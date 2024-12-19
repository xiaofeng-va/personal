#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    peripherals,
    peripherals::UART4,
    usart,
    usart::{BufferedUart, BufferedUartRx, Config},
};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use embedded_io_async::{Read, Write};
use panic_probe as _;

bind_interrupts!(struct Irqs {
    UART4 => usart::BufferedInterruptHandler<peripherals::UART4>;
});

// #[embassy_executor::task]
// async fn writer(mut usart: Uart<'static, UART4, NoDma, NoDma>) {
//     unwrap!(usart.blocking_write(b"Hello Embassy World!\r\n"));
//     info!("wrote Hello, starting echo");

//     let mut buf = [0u8; 1];
//     loop {
//         unwrap!(usart.blocking_read(&mut buf));
//         unwrap!(usart.blocking_write(&buf));
//     }
// }

static CHANNEL: Channel<ThreadModeRawMutex, [u8; 1], 1> = Channel::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    static mut TX_BUF: [u8; 256] = [0u8; 256];
    static mut RX_BUF: [u8; 256] = [0u8; 256];

    let config = Config::default();
    #[allow(static_mut_refs)]
    let usart = unsafe {
        BufferedUart::new(
            p.UART4,
            Irqs,
            p.PB8,
            p.PB9,
            &mut TX_BUF,
            &mut RX_BUF,
            config,
        )
        .unwrap()
    };
    let (mut tx, rx) = usart.split();

    unwrap!(spawner.spawn(reader(rx)));

    loop {
        let buf = CHANNEL.receive().await;
        info!("writing... {:?}", buf);
        unwrap!(tx.write_all(&buf).await);
        unwrap!(tx.write_all(&buf).await);
        info!("wrote... {:?}", buf);
    }
}

#[embassy_executor::task]
async fn reader(mut rx: BufferedUartRx<'static, UART4>) {
    let mut buf = [0; 1];
    loop {
        info!("reading...");
        unwrap!(rx.read(&mut buf).await);
        info!("read {:?}", buf);
        CHANNEL.send(buf).await;
    }
}
