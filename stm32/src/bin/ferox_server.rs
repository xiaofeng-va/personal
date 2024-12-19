#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    peripherals::{self, UART4, UART5},
    usart::{self, BufferedUart, BufferedUartRx, BufferedUartTx, Config},
};
use embassy_time::{Duration, Timer};
use embedded_io_async::Read;
use ferox_stm32::processor::{Processor, CHANNEL};
use panic_probe as _;

bind_interrupts!(struct Irqs1 {
    UART4 => usart::BufferedInterruptHandler<peripherals::UART4>;
});

bind_interrupts!(struct Irqs2 {
    UART5 => usart::BufferedInterruptHandler<peripherals::UART5>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    static mut TX_BUF1: [u8; 256] = [0u8; 256];
    static mut RX_BUF1: [u8; 256] = [0u8; 256];

    let config = Config::default();
    #[allow(static_mut_refs)]
    let usart = unsafe {
        BufferedUart::new(
            p.UART4,
            Irqs1,
            p.PB8,
            p.PB9,
            &mut TX_BUF1,
            &mut RX_BUF1,
            config,
        )
        .unwrap()
    };
    let (tx, rx) = usart.split();

    static mut TX_BUF2: [u8; 256] = [0u8; 256];
    static mut RX_BUF2: [u8; 256] = [0u8; 256];
    let config_ctl200 = Config::default();
    #[allow(static_mut_refs)]
    let usart_ctl200 = unsafe {
        BufferedUart::new(
            p.UART5,
            Irqs2,
            p.PB5,
            p.PB6,
            &mut TX_BUF2,
            &mut RX_BUF2,
            config_ctl200,
        )
        .unwrap()
    };

    let processor = Processor::new(tx, usart_ctl200);

    unwrap!(spawner.spawn(reader_task(rx)));
    unwrap!(spawner.spawn(processor_task(processor)));
    loop {
        Timer::after(Duration::from_millis(1000)).await;
    }
}

#[embassy_executor::task]
async fn reader_task(mut rx: BufferedUartRx<'static, UART4>) {
    let mut buf = [0; 1];
    loop {
        info!("reading...");
        unwrap!(rx.read(&mut buf).await);
        info!("read {:?}", buf);
        CHANNEL.send(buf).await;
    }
}

type ProcessorType<'a> = Processor<BufferedUartTx<'a, UART4>, BufferedUart<'a, UART5>>;

#[embassy_executor::task]
async fn processor_task(mut processor: ProcessorType<'static>) {
    loop {
        info!("Waiting for a message...");
        match processor.process_message().await {
            Ok(_) => info!("Message processed successfully"),
            Err(e) => error!("Error processing message: {:?}", e),
        }
    }
}
