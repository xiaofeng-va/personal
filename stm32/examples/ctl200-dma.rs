#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts, peripherals,
    peripherals::{DMA1_CH0, DMA1_CH1, UART7},
    usart,
    usart::{Config, Uart, UartRx, UartTx},
};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use embassy_time::Timer;
use ferox::{error, info, unwrap};
use heapless::Vec;
use panic_probe as _;
use static_cell::StaticCell;

bind_interrupts!(struct Irqs {
    UART7 => usart::InterruptHandler<peripherals::UART7>;
});

const CRLF: &[u8] = b"\r\n";
const VERSION_CMD: &[u8] = b"version\r\n";
const PROMPT: &[u8] = b">>";

static RX_BUFFER: StaticCell<[u8; 256]> = StaticCell::new();
static CHAR_CHANNEL: Channel<ThreadModeRawMutex, u8, 32> = Channel::new();

#[embassy_executor::task]
async fn uart_reader(uart: UartRx<'static, UART7, DMA1_CH1>) {
    info!("uart_reader started");

    let buffer = RX_BUFFER.init([0; 256]);
    let mut uart = uart.into_ring_buffered(buffer);
    let mut temp_buf = [0u8; 32];

    loop {
        match uart.read(&mut temp_buf).await {
            Ok(count) => {
                info!("uart_reader(): Read {} bytes", count);
                for &b in &temp_buf[..count] {
                    CHAR_CHANNEL.send(b).await;
                }
            }
            Err(e) => match e {
                usart::Error::Framing => error!("Framing error"),
                usart::Error::Noise => error!("Noise error"),
                usart::Error::Overrun => error!("Overrun error"),
                usart::Error::Parity => error!("Parity error"),
                _ => error!("Unknown UART error"),
            },
        }
    }
}

#[embassy_executor::task]
async fn char_processor(mut uart: UartTx<'static, UART7, DMA1_CH0>) {
    let mut buffer: Vec<u8, 32> = Vec::new();
    info!("char_processor started");

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

    uart.write(CRLF).await.unwrap();
    info!("Sent CRLF");
    Timer::after_millis(1000).await;

    wait_for_prompt(&mut buffer).await;
    info!(
        "Buffer content after wait_for_prompt: {:x}",
        buffer.as_slice()
    );

    uart.write(VERSION_CMD).await.unwrap();
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

    let (tx, rx) = uart.split();

    unwrap!(spawner.spawn(uart_reader(rx)));
    unwrap!(spawner.spawn(char_processor(tx)));

    loop {
        Timer::after_millis(1000).await;
    }
}
