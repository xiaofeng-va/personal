#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::usart::{BasicInstance, Config, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    UART7 => usart::InterruptHandler<peripherals::UART7>;
});

const CRLF: &[u8] = b"\r\n";
const VERSION_CMD: &[u8] = b"version\r\n";
const PROMPT: &[u8] = b">>";

async fn wait_for_prompt<'d, T: BasicInstance, TxDma, RxDma>(
    usart: &mut Uart<'d, T, TxDma, RxDma>
) -> bool {
    let mut rx_buf = [0u8; 2];
    unwrap!(usart.blocking_write(CRLF));
    info!("Sent CRLF");

    Timer::after(Duration::from_millis(1000)).await;

    match usart.blocking_read(&mut rx_buf) {
        Ok(()) if &rx_buf == PROMPT => {
            info!("Prompt received!");
            true
        }
        _ => {
            info!("No prompt received, retrying...");
            false
        }
    }
}

async fn read_response<'d, T: BasicInstance, TxDma, RxDma>(
    usart: &mut Uart<'d, T, TxDma, RxDma>
) {
    let mut byte = [0u8; 1];
    loop {
        match usart.blocking_read(&mut byte) {
            Ok(()) => {
                info!("Received char: '{}' (0x{=[u8]:x})", byte[0] as char, byte);
            }
            Err(e) => {
                info!("Read error: {:?}", Debug2Format(&e));
                Timer::after(Duration::from_millis(100)).await;
            }
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    info!("CTL200 Test Starting!");

    let config = Config::default();
    let mut usart = Uart::new(p.UART7, p.PF6, p.PF7, Irqs, p.DMA1_CH0, p.DMA1_CH1, config).unwrap();

    loop {
        if wait_for_prompt(&mut usart).await {
            unwrap!(usart.blocking_write(VERSION_CMD));
            info!("Sent version command");
            read_response(&mut usart).await;
        }
        Timer::after(Duration::from_millis(1000)).await;
    }
}
