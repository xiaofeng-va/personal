#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::peripherals::{DMA1_CH0, DMA1_CH1, UART7};
use embassy_stm32::usart::{Config, Uart, UartRx, UartTx};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::Timer;
use heapless::Vec;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    UART7 => usart::InterruptHandler<peripherals::UART7>;
});

const CRLF: &[u8] = b"\r\n";
const VERSION_CMD: &[u8] = b"version\r\n";
const PROMPT: &[u8] = b">>";

static CHAR_CHANNEL: Channel<ThreadModeRawMutex, u8, 32> = Channel::new();

#[embassy_executor::task]
async fn uart_reader(mut uart: UartRx<'static, UART7, DMA1_CH1>) {
    info!("uart_reader started");
    loop {
        let mut buffer: Vec<u8, 32> = Vec::new();
        loop {
            match uart.nb_read() {
                Ok(x) => {
                    // info!("ok {}", x);
                    buffer.push(x).unwrap();
                }
                Err(e) => {
                    // info!("err: {}", Debug2Format(&e));
                    break;
                }
            }
            // Adding a short delay to ensure nb_read() can read the last character,
            // otherwise it might return before the character arrives, causing it to be missed.
            Timer::after_micros(1).await;
        }
        if buffer.len() > 0 {
            info!("uart_reader(): Read buffer: {:x}", buffer.as_slice());
            for &b in buffer.iter() {
                CHAR_CHANNEL.send(b).await;
            }
        }
        Timer::after_millis(100).await;
    }
}

#[embassy_executor::task]
async fn char_processor(mut uart: UartTx<'static, UART7, DMA1_CH0>) {
    let mut buffer: Vec<u8, 32> = Vec::new();
    info!("char_processor started");
    // Initial loop to send CRLF until PROMPT is received
    async fn wait_for_prompt(buffer: &mut Vec<u8, 32>) {
        buffer.clear();
        loop {
            let char = CHAR_CHANNEL.receive().await;
            buffer.extend_from_slice(&[char]).unwrap();

            if buffer.ends_with(PROMPT) {
                return;
            }

            if buffer.len() >= 32 {
                info!("Buffer overflow, content: {:x}", buffer.as_slice());
                buffer.clear();
            }
        }
    }

    unwrap!(uart.blocking_write(CRLF));
    info!("Sent CRLF");
    Timer::after_millis(1000).await;
    wait_for_prompt(&mut buffer).await;
    info!(
        "Buffer content after wait_for_prompt: {:x}",
        buffer.as_slice()
    );
    unwrap!(uart.blocking_write(VERSION_CMD));
    info!("Sent 'version' command after initial loop");
    wait_for_prompt(&mut buffer).await;
    info!("PASS");
    loop {
        Timer::after_millis(1000).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    info!("CTL200 Test Starting!");

    let config = Config::default();
    let uart = Uart::new(p.UART7, p.PF6, p.PF7, Irqs, p.DMA1_CH0, p.DMA1_CH1, config).unwrap();

    // Split UART for different tasks
    let (tx, rx) = uart.split();

    unwrap!(spawner.spawn(uart_reader(rx)));
    unwrap!(spawner.spawn(char_processor(tx)));

    loop {
        Timer::after_millis(1000).await;
    }
}
