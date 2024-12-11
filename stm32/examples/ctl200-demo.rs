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
use num_traits::float::FloatCore;

#[cortex_m_rt::exception]
unsafe fn HardFault(frame: &cortex_m_rt::ExceptionFrame) -> ! {
    loop {
        // defmt::error!("HardFault at {:#?}", frame);
        debug::exit(debug::EXIT_FAILURE);
    }
}

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

    {
        let lason = ctl200.laser_en().await?;
        info!("Laser is {}", if lason { "ON" } else { "OFF" });
        ctl200.set_laser_en(!lason).await?;
        let lason2 = ctl200.laser_en().await?;
        info!("Laser is {}", if lason2 { "ON" } else { "OFF" });
        if lason2 == lason {
            return Err(Error::WriteError);
        }
    }
    info!("1 | Current SP: 0x{:X}", cortex_m::register::msp::read());

    {
        let laser_current_mA = ctl200.laser_current_mA().await?;
        info!("Laser current is {} mA", laser_current_mA);
        ctl200.set_laser_current_mA(laser_current_mA + 1.0f32).await?;
        let laser_current_mA2 = ctl200.laser_current_mA().await?;
        info!("New laser current is {} mA", laser_current_mA2);
        if (laser_current_mA2 - laser_current_mA).abs() < 0.1f32 {
            return Err(Error::WriteError);
        }
    }
    info!("2 | Current SP: 0x{:X}", cortex_m::register::msp::read());

    {
        let laser_V = ctl200.laser_V().await?;
        info!("Laser voltage is {} V", laser_V);
    }

    {
        let laser_delay_ms = ctl200.laser_delay_ms().await?;
        info!("Laser delay is {} ms", laser_delay_ms);
        ctl200.set_laser_delay_ms(laser_delay_ms + 1.0f32).await?;
        let laser_delay_ms2 = ctl200.laser_delay_ms().await?;
        info!("New laser delay is {} ms", laser_delay_ms2);
        if (laser_delay_ms2 - laser_delay_ms).abs() < 0.1f32 {
            return Err(Error::WriteError);
        }
    }

    {
        let current_limit_mA = ctl200.current_limit_mA().await?;
        info!("Current limit is {} mA", current_limit_mA);
        ctl200.set_current_limit_mA(current_limit_mA + 1.0f32).await?;
        let current_limit_mA2 = ctl200.current_limit_mA().await?;
        info!("New limit is {} mA", current_limit_mA2);
        // if (current_limit_mA2 - current_limit_mA).abs() < 0.1f32 {
        //     return Err(Error::WriteError);
        // }
    }

    {
        let interlock_en = ctl200.interlock_en().await?;
        info!("Interlock is {}", if interlock_en { "ON" } else { "OFF" });
        ctl200.set_interlock_en(!interlock_en).await?;
        let interlock_en2 = ctl200.interlock_en().await?;
        info!("New Interlock is {}", if interlock_en2 { "ON" } else { "OFF" });
        if interlock_en2 == interlock_en {
            return Err(Error::WriteError);
        }
    }

    {
        let current_mod_gain_mA_V = ctl200.laser_current_mod_gain_mA_V().await?;
        info!("Current laser modulation gain is {} mA/V", current_mod_gain_mA_V);
        ctl200.set_laser_current_mod_gain_mA_V(current_mod_gain_mA_V + 1.0f32).await?;
        let current_mod_gain_mA_V2 = ctl200.laser_current_mod_gain_mA_V().await?;
        info!("New laser modulation gain is {} mA/V", current_mod_gain_mA_V2);
        if (current_mod_gain_mA_V2 - current_mod_gain_mA_V).abs() < 0.1f32 {
            return Err(Error::WriteError);
        }            
    }

    {
        let tec = ctl200.tec_en().await?;
        info!("TEC is {}", if tec { "ON" } else { "OFF" });
        ctl200.set_tec_en(!tec).await?;
        let tec2 = ctl200.tec_en().await?;
        info!("New TEC is {}", if tec2 { "ON" } else { "OFF" });
        if tec2 == tec {
            return Err(Error::WriteError);
        }
    }
    
    {
        let temp_protect = ctl200.temp_prot_en().await?;
        info!("Temperature protection is {}", if temp_protect { "ON" } else { "OFF" });
        ctl200.set_temp_prot_en(!temp_protect).await?;
        let temp_protect2 = ctl200.temp_prot_en().await?;
        info!("New temperature protection is {}", if temp_protect2 { "ON" } else { "OFF" });
        if temp_protect2 == temp_protect {
            return Err(Error::WriteError);
        }
    }

    {
        let temp_set_Ohm = ctl200.temp_set_Ohm().await?;
        info!("Temperature setpoint is {} Ohm", temp_set_Ohm);
        ctl200.set_temp_set_Ohm(temp_set_Ohm + 1.0f32).await?;  
        let temp_set_Ohm2 = ctl200.temp_set_Ohm().await?;
        info!("New temperature setpoint is {} Ohm", temp_set_Ohm2);
        if (temp_set_Ohm2 - temp_set_Ohm).abs() < 0.1f32 {
            return Err(Error::WriteError);
        }
    }

    {
        let temp_act_Ohm = ctl200.temp_act_Ohm().await?;
        info!("Actual temperature is {} Ohm", temp_act_Ohm);
    }

    {
        let tec_current_A = ctl200.tec_current_A().await?;
        info!("Current is {} A", tec_current_A);
        let tec_voltage_V = ctl200.tec_voltage_V().await?;
        info!("Voltage is {} V", tec_voltage_V);
    }

    {
        let prop_gain = ctl200.prop_gain().await?;
        info!("Proportional gain is {}", prop_gain);
        ctl200.set_prop_gain(prop_gain + 1.0f32).await?;
        let prop_gain2 = ctl200.prop_gain().await?;
        info!("New proportional gain is {}", prop_gain2);
        // if (prop_gain2 - prop_gain).abs() < 0.1f32 {
        //     return Err(Error::WriteError);
        // }
    }
    info!("8 | Current SP: 0x{:X}", cortex_m::register::msp::read());

    {
        let int_gain = ctl200.int_gain().await?;
        info!("Integral gain is {}", int_gain);
        ctl200.set_int_gain(int_gain + 1.0f32).await?;
        let int_gain2 = ctl200.int_gain().await?;
        info!("New integral gain is {}", int_gain2);
        // if (int_gain2 - int_gain).abs() < 0.1f32 {
        //     return Err(Error::WriteError);
        // }
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
    use core::mem;
    
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
