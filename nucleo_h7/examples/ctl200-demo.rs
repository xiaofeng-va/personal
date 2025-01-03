#![no_std]
#![no_main]

use core::default::Default;

use defmt::{error, info};
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts, peripherals,
    usart::{self, BufferedUart, Config},
};
use embassy_time::Timer;
use ferox::{drivers::koheron::ctl200::Ctl200, proto::error::Error};
use num_traits::float::FloatCore;
use panic_halt as _;

bind_interrupts!(struct Irqs {
    UART7 => usart::BufferedInterruptHandler<peripherals::UART7>;
});

type CTL200 = Ctl200<BufferedUart<'static, peripherals::UART7>>;

#[allow(non_snake_case)]
async fn ctl200_process(mut ctl200: CTL200) -> Result<(), Error> {
    if ctl200.version().await? != b"V0.17" {
        return Err(Error::Ctl200InvalidResponse);
    }

    {
        let lason = ctl200.laser_en().await?;
        info!("Laser is {}", if lason { "ON" } else { "OFF" });
        ctl200.set_laser_en(!lason).await?;
        let lason2 = ctl200.laser_en().await?;
        info!("Laser is {}", if lason2 { "ON" } else { "OFF" });
        let _ = ctl200.set_laser_en(lason).await; // reset
        if lason2 == lason {
            return Err(Error::Ctl200InvalidResponse);
        }
    }

    {
        let laser_current_mA = ctl200.laser_current_mA().await?;
        info!("Laser current is {} mA", laser_current_mA);
        ctl200
            .set_laser_current_mA(laser_current_mA + 1.0f32)
            .await?;
        let laser_current_mA2 = ctl200.laser_current_mA().await?;
        info!("New laser current is {} mA", laser_current_mA2);
        let _ = ctl200.set_laser_current_mA(laser_current_mA).await; // reset
        if (laser_current_mA2 - laser_current_mA - 1f32).abs() > 0.1f32 {
            return Err(Error::Ctl200InvalidResponse);
        }
    }

    {
        let laser_delay_ms = ctl200.laser_delay_ms().await?;
        info!("Laser delay is {} ms", laser_delay_ms);
        ctl200.set_laser_delay_ms(laser_delay_ms + 1.0f32).await?;
        let laser_delay_ms2 = ctl200.laser_delay_ms().await?;
        info!("New laser delay is {} ms", laser_delay_ms2);
        let _ = ctl200.set_laser_delay_ms(laser_delay_ms).await; // reset
        if (laser_delay_ms2 - laser_delay_ms - 1f32).abs() > 0.1f32 {
            return Err(Error::Ctl200InvalidResponse);
        }
    }

    {
        let current_limit_mA = ctl200.current_limit_mA().await?;
        info!("Current limit is {} mA", current_limit_mA);
        ctl200
            .set_current_limit_mA(current_limit_mA + 1.0f32)
            .await?;
        let current_limit_mA2 = ctl200.current_limit_mA().await?;
        info!("New limit is {} mA", current_limit_mA2);
        let _ = ctl200.set_current_limit_mA(current_limit_mA).await; // reset
    }

    {
        let interlock_en = ctl200.interlock_en().await?;
        info!("Interlock is {}", if interlock_en { "ON" } else { "OFF" });
        ctl200.set_interlock_en(!interlock_en).await?;
        let interlock_en2 = ctl200.interlock_en().await?;
        info!(
            "New Interlock is {}",
            if interlock_en2 { "ON" } else { "OFF" }
        );
        let _ = ctl200.set_interlock_en(interlock_en).await; // reset
        if interlock_en2 == interlock_en {
            return Err(Error::Ctl200InvalidResponse);
        }
    }

    {
        let current_mod_gain_mA_V = ctl200.laser_current_mod_gain_mA_V().await?;
        info!(
            "Current laser modulation gain is {} mA/V",
            current_mod_gain_mA_V
        );
        ctl200
            .set_laser_current_mod_gain_mA_V(current_mod_gain_mA_V + 1.0f32)
            .await?;
        let current_mod_gain_mA_V2 = ctl200.laser_current_mod_gain_mA_V().await?;
        info!(
            "New laser modulation gain is {} mA/V",
            current_mod_gain_mA_V2
        );
        let _ = ctl200
            .set_laser_current_mod_gain_mA_V(current_mod_gain_mA_V)
            .await; // reset
        if (current_mod_gain_mA_V2 - current_mod_gain_mA_V - 1f32).abs() > 0.1f32 {
            return Err(Error::Ctl200InvalidResponse);
        }
    }

    {
        let tec = ctl200.tec_en().await?;
        info!("TEC is {}", if tec { "ON" } else { "OFF" });
        ctl200.set_tec_en(!tec).await?;
        let tec2 = ctl200.tec_en().await?;
        info!("New TEC is {}", if tec2 { "ON" } else { "OFF" });
        let _ = ctl200.set_tec_en(tec).await; // reset
        if tec2 == tec {
            return Err(Error::Ctl200InvalidResponse);
        }
    }

    {
        let temp_protect = ctl200.temp_prot_en().await?;
        info!(
            "Temperature protection is {}",
            if temp_protect { "ON" } else { "OFF" }
        );
        ctl200.set_temp_prot_en(!temp_protect).await?;
        let temp_protect2 = ctl200.temp_prot_en().await?;
        info!(
            "New temperature protection is {}",
            if temp_protect2 { "ON" } else { "OFF" }
        );
        let _ = ctl200.set_temp_prot_en(temp_protect).await; // reset
        if temp_protect2 == temp_protect {
            return Err(Error::Ctl200InvalidResponse);
        }
    }

    {
        let temp_set_Ohm = ctl200.temp_set_Ohm().await?;
        info!("Temperature setpoint is {} Ohm", temp_set_Ohm);
        ctl200.set_temp_set_Ohm(temp_set_Ohm + 1.0f32).await?;
        let temp_set_Ohm2 = ctl200.temp_set_Ohm().await?;
        info!("New temperature setpoint is {} Ohm", temp_set_Ohm2);
        let _ = ctl200.set_temp_set_Ohm(temp_set_Ohm).await; // reset
        if (temp_set_Ohm2 - temp_set_Ohm - 1f32).abs() > 0.1f32 {
            return Err(Error::Ctl200InvalidResponse);
        }
    }

    {
        let prop_gain = ctl200.prop_gain().await?;
        info!("Proportional gain is {}", prop_gain);
        ctl200.set_prop_gain(prop_gain + 1.0f32).await?;
        let prop_gain2 = ctl200.prop_gain().await?;
        info!("New proportional gain is {}", prop_gain2);
        let _ = ctl200.set_prop_gain(prop_gain).await; // reset
    }

    {
        let int_gain = ctl200.int_gain().await?;
        info!("Integral gain is {}", int_gain);
        ctl200.set_int_gain(int_gain + 1.0f32).await?;
        let int_gain2 = ctl200.int_gain().await?;
        info!("New integral gain is {}", int_gain2);
        let _ = ctl200.set_int_gain(int_gain).await; // reset
    }

    {
        let diff_gain = ctl200.diff_gain().await?;
        info!("Differential gain is {}", diff_gain);
        ctl200.set_diff_gain(diff_gain + 1.0f32).await?;
        let diff_gain2 = ctl200.diff_gain().await?;
        info!("New differential gain is {}", diff_gain2);
        let _ = ctl200.set_diff_gain(diff_gain).await; // reset
    }

    {
        let temp_min_Ohm = ctl200.temp_min_Ohm().await?;
        info!("Minimum temperature is {} Ohm", temp_min_Ohm);
        ctl200.set_temp_min_Ohm(temp_min_Ohm + 1.0f32).await?;
        let temp_min_Ohm2 = ctl200.temp_min_Ohm().await?;
        info!("New minimum temperature is {} Ohm", temp_min_Ohm2);
        let _ = ctl200.set_temp_min_Ohm(temp_min_Ohm).await; // reset
        if (temp_min_Ohm2 - temp_min_Ohm - 1f32).abs() > 0.1f32 {
            return Err(Error::Ctl200InvalidResponse);
        }
    }

    {
        let temp_max_Ohm = ctl200.temp_max_Ohm().await?;
        info!("Maximum temperature is {} Ohm", temp_max_Ohm);
        ctl200.set_temp_max_Ohm(temp_max_Ohm + 1.0f32).await?;
        let temp_max_Ohm2 = ctl200.temp_max_Ohm().await?;
        info!("New maximum temperature is {} Ohm", temp_max_Ohm2);
        let _ = ctl200.set_temp_max_Ohm(temp_max_Ohm).await; // reset
        if (temp_max_Ohm2 - temp_max_Ohm - 1f32).abs() > 0.1f32 {
            return Err(Error::Ctl200InvalidResponse);
        }
    }

    {
        let tec_min_V = ctl200.tec_min_V().await?;
        info!("Minimum TEC voltage is {} V", tec_min_V);
        ctl200.set_tec_min_V(tec_min_V + 1.0f32).await?;
        let tec_min_V2 = ctl200.tec_min_V().await?;
        info!("New minimum TEC voltage is {} V", tec_min_V2);
        let _ = ctl200.set_tec_min_V(tec_min_V).await; // reset
        if (tec_min_V2 - tec_min_V - 1f32).abs() > 0.1f32 {
            return Err(Error::Ctl200InvalidResponse);
        }
    }

    {
        let tec_max_V = ctl200.tec_max_V().await?;
        info!(
            "Maximum TEC voltage is {} V, {}",
            tec_max_V,
            tec_max_V + 1f32
        );
        ctl200.set_tec_max_V(tec_max_V + 1f32).await?;
        let tec_max_V2 = ctl200.tec_max_V().await?;
        info!("New maximum TEC voltage is {} V", tec_max_V2);
        let _ = ctl200.set_tec_max_V(tec_max_V).await; // reset
                                                       // TODO(xguo): This one is really weird, while "vtmax 4.0" will
                                                       // set this to 2.0 Will double check next.
    }

    {
        let temp_mod_gain_Ohm_V = ctl200.temp_mod_gain_Ohm_V().await?;
        info!(
            "Temperature modulation gain is {} Ohm/V",
            temp_mod_gain_Ohm_V
        );
        ctl200
            .set_temp_mod_gain_Ohm_V(temp_mod_gain_Ohm_V + 1.0f32)
            .await?;
        let temp_mod_gain_Ohm_V2 = ctl200.temp_mod_gain_Ohm_V().await?;
        info!(
            "New temperature modulation gain is {} Ohm/V",
            temp_mod_gain_Ohm_V2
        );
        let _ = ctl200.set_temp_mod_gain_Ohm_V(temp_mod_gain_Ohm_V).await?; // reset
        if (temp_mod_gain_Ohm_V2 - temp_mod_gain_Ohm_V - 1f32).abs() > 0.1f32 {
            return Err(Error::Ctl200InvalidResponse);
        }
    }

    {
        let temp_act_Ohm = ctl200.temp_act_Ohm().await?;
        info!("Actual temperature is {} Ohm", temp_act_Ohm);
        let tec_current_A = ctl200.tec_current_A().await?;
        info!("Current is {} A", tec_current_A);
        let tec_voltage_V = ctl200.tec_voltage_V().await?;
        info!("Voltage is {} V", tec_voltage_V);
        let laser_V: f32 = ctl200.laser_V().await?;
        info!("Laser voltage is {} V", laser_V);
        let ain_1_V = ctl200.ain_1_V().await?;
        info!("AIN 1 is {} V", ain_1_V);
        let ain_2_V = ctl200.ain_2_V().await?;
        info!("AIN 2 is {} V", ain_2_V);
        let board_temp_C = ctl200.board_temp_C().await?;
        info!("Board temperature is {} C", board_temp_C);
        // let board_status = ctl200.board_status().await?;
        // info!("Board status is 0x{:X}", board_status);
        let serial_number = ctl200.serial_number().await?;
        info!("Serial number is {}", serial_number);
    }

    {
        let userdata = ctl200.userdata().await?;
        info!("User data is {}", userdata);
        let userdata = b"hello";
        ctl200.set_userdata(userdata).await?;
        let userdata2 = ctl200.userdata().await?;
        info!("New user data is {}", userdata2);
        if userdata2 != b"hello" {
            return Err(Error::Ctl200InvalidResponse);
        }
    }

    {
        let baud_rate_Hz = ctl200.baud_rate_Hz().await?;
        info!("Baud rate is {} Hz", baud_rate_Hz);
        ctl200.set_baud_rate_Hz(baud_rate_Hz - 10).await?;
        let baud_rate_Hz2 = ctl200.baud_rate_Hz().await?;
        info!("New baud rate is {} Hz", baud_rate_Hz2);
        let _ = ctl200.set_baud_rate_Hz(baud_rate_Hz).await; // reset
    }

    {
        // untested: err / clear_err / save_config
        // Too long string: board_status
        // Unknown: vtmax
    }

    Ok(())
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
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
