#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts, peripherals::{self, UART4},
    usart::{self, BufferedUart, BufferedUartRx, BufferedUartTx, Config},
};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use embedded_io_async::{Read, Write};
use ferox::{drivers::koheron::ctl200, proto::{data::FeroxProto, errors::Error}, MAX_STRING_SIZE};
use ferox_stm32::handler::{handle_ctl200_request, handle_ferox_request, handle_request};
use heapless::Vec;
use panic_probe as _;
use embassy_time::{Duration, Timer};

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
    unwrap!(spawner.spawn(processor(tx)));

    loop {
        Timer::after(Duration::from_millis(1000)).await;
    }
}

async fn process_message(tx: &mut BufferedUartTx<'_, UART4>) -> ferox::proto::errors::Result<()> {
    // Read the size byte
    let size = CHANNEL.receive().await[0] as usize;

    // Read the content based on the size
    let mut content_buf = [0u8; 256];
    let content_buf = &mut content_buf[..size];
    for byte in content_buf.iter_mut() {
        *byte = CHANNEL.receive().await[0];
    }

    let req = postcard::from_bytes::<FeroxProto>(&content_buf).map_err(|_| Error::PostcardDeserializeError)?;
    let resp = handle_request(req).await?;

    let mut buf = [0u8; 256];
    let resp_bytes = postcard::to_slice(&resp, &mut buf).map_err(|_| Error::PostcardSerializeError)?;
    let resp_size = resp_bytes.len() as u8;
    tx.write_all(&[resp_size]).await.map_err(|_| Error::WriteError)?;
    tx.write_all(&resp_bytes).await.map_err(|_| Error::WriteError)?;

    Ok(())
}

#[embassy_executor::task]
async fn processor(mut tx: BufferedUartTx<'static, UART4>) {
    loop {
        info!("Waiting for a message...");
        match process_message(&mut tx).await {
            Ok(_) => info!("Message processed successfully"),
            Err(e) => warn!("Error processing message: {:?}", e),
        }
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
