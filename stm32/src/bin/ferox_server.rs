#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts, peripherals,
    peripherals::UART4,
    usart,
    usart::{BufferedUart, BufferedUartRx, Config},
};
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use embedded_io_async::{Read, Write};
use ferox::{drivers::koheron::ctl200, proto::{data::FeroxProto, errors::Error}};
use ferox_stm32::handler::{handle_ctl200_request, handle_ferox_request};
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

async fn process_message(buf: &[u8]) -> ferox::proto::errors::Result<FeroxProto> {
    // Read the size byte
    let size = CHANNEL.receive().await[0] as usize;

    // Read the content based on the size
    let mut content_buf = [0u8; 256];
    let content_buf = &mut content_buf[..size];
    for byte in content_buf.iter_mut() {
        *byte = CHANNEL.receive().await[0];
    }

    match postcard::from_bytes::<FeroxProto>(&content_buf) {
        Ok(FeroxProto::FeroxRequest(ferox_req)) => {
            Ok(FeroxProto::FeroxResponse(handle_ferox_request(&ferox_req)?))
        }
        Ok(FeroxProto::Ctl200Request(ctl200_req)) => {
            Ok(FeroxProto::Ctl200Response(handle_ctl200_request(&ctl200_req)?))
        }
        Err(e) => {
            // TODO(xguo): Enable defmt-or-log and add error_id to the log.
            warn!("Failed to deserialize FeroxProto");
            Err(Error::PostcardDeserializeError)
        }
        _ => {
            warn!("Received an unexpected FeroxProto, {}", content_buf);
            Err(Error::UnexpectedFeroxRequest)
        }
    }
}

#[embassy_executor::task]
async fn processor() {
    loop {
        info!("Waiting for a message...");
        match process_message(&[]).await {
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
