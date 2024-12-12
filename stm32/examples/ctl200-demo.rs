#![no_std]
#![no_main]

use core::default::Default;

use cortex_m_semihosting::debug;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    dma::NoDma,
    peripherals,
    usart::{self, BasicInstance, BufferedUart, Config, RingBufferedUartRx, Uart, UartTx},
};
use embassy_time::Timer;
use embedded_io::ErrorType;
use embedded_io_async::{Read, Write};
use ferox::{
    drivers::koheron::ctl200::{Ctl200, Error},
    error, info, debug,
};
use panic_halt as _;
use static_cell::StaticCell;

bind_interrupts!(struct Irqs {
    UART7 => usart::BufferedInterruptHandler<peripherals::UART7>;
});

type CTL200 = Ctl200<'static, BufferedUart<'static, peripherals::UART7>>;
async fn ctl200_process(mut ctl200: CTL200) -> Result<(), Error> {
    if ctl200.version().await? != b"V0.17" {
        return Err(Error::InvalidFirmwareVersion);
    }
    Ok(())
}

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    info!("CTL200 Example Starting!");
    static mut TX_BUF: [u8; 256] = [0u8; 256];
    static mut RX_BUF: [u8; 256] = [0u8; 256];

    let config = Config::default();
    #[allow(static_mut_refs)]
    let usart = unsafe {
        BufferedUart::new(
            p.UART7,
            Irqs,
            p.PF6,
            p.PF7,
            &mut TX_BUF,
            &mut RX_BUF,
            config,
        )
        .unwrap()
    };

    let ctl200 = Ctl200::new(usart);
    match ctl200_process(ctl200).await {
        Ok(_) => {
            info!("CTL200 Example Finished!");
        }
        Err(e) => {
            error!("CTL200 Example Failed, err: {}!", e);
        }
    }

    loop {
        info!("In main loop...");
        Timer::after_millis(1000).await
    }
}
