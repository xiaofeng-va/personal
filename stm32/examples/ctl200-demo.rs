#![no_std]
#![no_main]

use core::default::Default;

use cortex_m_semihosting::debug;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    dma::NoDma,
    peripherals, usart,
    usart::{BasicInstance, Config, RingBufferedUartRx, Uart, UartTx},
};
use embassy_time::Timer;
use embedded_io::ErrorType;
use embedded_io_async::{Read, Write};
use ferox::{
    drivers::koheron::ctl200::{Ctl200, Error},
    error, info,
};
use ferox_stm32::ctl200::FIRMWARE_VERSION;
use panic_halt as _;
use static_cell::StaticCell;

bind_interrupts!(struct Irqs {
    UART7 => usart::InterruptHandler<peripherals::UART7>;
});

static RX_BUFFER: StaticCell<[u8; 256]> = StaticCell::new();

pub struct UartWrapper<'d, T: BasicInstance, TxDma = NoDma, RxDma = NoDma>
where
    RxDma: usart::RxDma<T>,
{
    tx: UartTx<'d, T, TxDma>,
    rx: RingBufferedUartRx<'d, T, RxDma>,
}

impl<'d, T: BasicInstance, TxDma: usart::TxDma<T>, RxDma: usart::RxDma<T>>
    UartWrapper<'d, T, TxDma, RxDma>
{
    pub fn new(uart: Uart<'d, T, TxDma, RxDma>) -> Self {
        let (tx, rx0) = uart.split();

        let buffer = RX_BUFFER.init([0; 256]);
        let rx = rx0.into_ring_buffered(buffer);
        Self { tx, rx }
    }
}

impl<'d, T: BasicInstance, TxDma, RxDma: usart::RxDma<T>> ErrorType
    for UartWrapper<'d, T, TxDma, RxDma>
{
    type Error = usart::Error;
}

impl<'d, T: BasicInstance, TxDma: usart::TxDma<T>, RxDma: usart::RxDma<T>> Read
    for UartWrapper<'d, T, TxDma, RxDma>
{
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.rx.read(buf).await.map(|_| buf.len())
    }
}

impl<'d, T: BasicInstance, TxDma: usart::TxDma<T>, RxDma: usart::RxDma<T>> Write
    for UartWrapper<'d, T, TxDma, RxDma>
{
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.tx.write(buf).await.map(|_| buf.len())
    }
}

type CTL200 =
    Ctl200<UartWrapper<'static, peripherals::UART7, peripherals::DMA1_CH0, peripherals::DMA1_CH1>>;
async fn ctl200_process(mut ctl200: CTL200) -> Result<(), Error> {
    if ctl200.version().await?.as_str() != FIRMWARE_VERSION {
        return Err(Error::ReadError);
    }
    let lason = ctl200.laser_en().await?;
    info!("Laser is {}", if lason { "ON" } else { "OFF" });
    ctl200.set_laser_en(!lason).await?;
    let lason2 = ctl200.laser_en().await?;
    info!("Laser is {}", if lason2 { "ON" } else { "OFF" });
    if lason2 == lason {
        return Err(Error::WriteError);
    }
    Ok(())
}

#[embassy_executor::task]
async fn ctl200_task(
    ctl200: Ctl200<
        UartWrapper<'static, peripherals::UART7, peripherals::DMA1_CH0, peripherals::DMA1_CH1>,
    >,
) -> ! {
    info!("Requesting CTL200 firmware version...");
    match ctl200_process(ctl200).await {
        Ok(_) => {
            info!("CTL200 testing process PASS");
            debug::exit(debug::EXIT_SUCCESS);
        }
        Err(e) => {
            error!("Failed in running the CTL200 testing process: {:?}", e);
            debug::exit(debug::EXIT_FAILURE);
        }
    }

    info!("Entering main loop...");
    loop {}
}

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    info!("CTL200 Example Starting!");

    let config = Config::default();
    let uart = Uart::new(p.UART7, p.PF6, p.PF7, Irqs, p.DMA1_CH0, p.DMA1_CH1, config).unwrap();
    let uart_wrapper = UartWrapper::new(uart);
    let ctl200: Ctl200<
        UartWrapper<'_, peripherals::UART7, peripherals::DMA1_CH0, peripherals::DMA1_CH1>,
    > = Ctl200::new(uart_wrapper);
    let _ = spawner.spawn(ctl200_task(ctl200)).unwrap();

    loop {
        Timer::after_millis(1000).await;
    }
}
