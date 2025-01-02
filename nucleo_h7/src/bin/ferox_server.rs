#![no_std]
#![no_main]

use defmt::{debug, error, info};
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts, peripherals,
    peripherals::{UART4, UART5, UART7},
    usart,
    usart::{BufferedUart, Config},
};
use embassy_time::{Duration, Timer};
use embedded_io_async::{Read, Write};
use ferox::{
    drivers::koheron::ctl200::Ctl200Request,
    proto::{
        ascii::{from_bytes, to_bytes},
        error::Error,
        ferox::{FeroxRequest, SmcRequest},
        Result,
    },
    uart::{
        post_processor::{PostProcessor, VaPostProcessor},
        read_until, UartWrapper,
    },
    MAX_STRING_SIZE,
};
use heapless::String;
use panic_probe as _;

bind_interrupts!(struct Irqs4 {
    UART4 => usart::BufferedInterruptHandler<peripherals::UART4>;
});

bind_interrupts!(struct Irqs5 {
    UART5 => usart::BufferedInterruptHandler<peripherals::UART5>;
});

bind_interrupts!(struct Irqs7 {
    UART7 => usart::BufferedInterruptHandler<peripherals::UART7>;
});

pub const CMD_PROMPT: &[u8] = b"\r\n";
pub const CTL200_END: &[u8] = b"\r\n>>";
pub const SMC_END: &[u8] = b"\n\r\n";

const MAX_RETRIES: i32 = 3;
const DEFAULT_TIMEOUT: Duration = Duration::from_millis(3_000);

pub struct FeroxServer<U0, U1, U2, P0, P1, P2> {
    /// UART4 is used to receive/send commands from the host or external devices
    controller: UartWrapper<U0, P0>,
    /// UART5 is used to communicate with the ctl200 device
    ctl200: UartWrapper<U1, P1>,
    /// UART7 is used to communicate with the smc device
    smc: UartWrapper<U2, P2>,
}

type UW<U, P> = UartWrapper<U, P>;

impl<U0, U1, U2, P0, P1, P2> FeroxServer<U0, U1, U2, P0, P1, P2>
where
    U0: Read + Write,
    U1: Read + Write,
    U2: Read + Write,
    P0: PostProcessor,
    P1: PostProcessor,
    P2: PostProcessor,
{
    pub fn new(controller: UW<U0, P0>, ctl200: UW<U1, P1>, smc: UW<U2, P2>) -> Self {
        Self {
            controller,
            ctl200,
            smc,
        }
    }

    async fn handle_all_versions(&mut self) -> Result<()> {
        info!("Handling AllVersions request");
        let ctl200_req_str =
            to_bytes(&Ctl200Request::Version).map_err(|_| Error::Ctl200RequestSerializeError)?;
        let smc_req_str =
            to_bytes(&SmcRequest::Version(None)).map_err(|_| Error::SmcRequestSerializeError)?;

        // 1. Send to ctl200
        let mut response_buf = [0u8; MAX_STRING_SIZE];
        debug!(
            "Querying CTL200 version with request: {:?}",
            core::str::from_utf8(&ctl200_req_str).unwrap_or("<invalid>")
        );
        let ctl_processed_resp = self
            .ctl200
            .query_with_pattern(
                &ctl200_req_str,
                CTL200_END,
                &mut response_buf,
                DEFAULT_TIMEOUT,
                MAX_RETRIES,
            )
            .await?;
        let ctl200_ver =
            from_bytes::<&[u8]>(ctl_processed_resp).map_err(|_| Error::InvalidResponse)?;
        debug!(
            "CTL200 version response: {:?}",
            core::str::from_utf8(ctl200_ver).unwrap_or("<invalid>")
        );

        // 2. Send to SMC.
        // TODO(xguo): You can use async-std::future::join() to send them concurrently.
        let mut smc_response_buf = [0u8; MAX_STRING_SIZE];
        debug!("Querying SMC version");
        let smc_processed_resp = self
            .smc
            .query_with_pattern(
                &smc_req_str,
                SMC_END,
                &mut smc_response_buf,
                DEFAULT_TIMEOUT,
                MAX_RETRIES,
            )
            .await?;
        let smc_ver =
            from_bytes::<&[u8]>(smc_processed_resp).map_err(|_| Error::InvalidResponse)?;
        debug!(
            "SMC version response: {:?}",
            core::str::from_utf8(smc_ver).unwrap_or("<invalid>")
        );

        // 3. Assemble the final string
        let mut resp_buf: String<MAX_STRING_SIZE> = String::new();
        {
            use core::{fmt::Write, str::from_utf8};
            // TODO(xguo): Replace the output part below with a function, and use the same interface for both success and error returns. In this function, we can also try using a small 128-byte string first, then switch to a larger 1024-byte string if needed. Most of our responses can fit in small strings, with only a few requiring larger strings. We need to handle this situation. Perhaps macros would be a good choice.
            info!(
                "Assembling response: CTL200: {:?}, SMC: {:?}",
                from_utf8(ctl200_ver).unwrap(),
                from_utf8(smc_ver).unwrap()
            );
            write!(
                resp_buf,
                "<ctl200>\r\n{}\r\n<smc>\r\n{}",
                from_utf8(ctl200_ver).unwrap_or("<invalid>"),
                from_utf8(smc_ver).unwrap_or("<invalid>"),
            )
            .map_err(|_| Error::FormatErrorInWriteResponse)?;
        }

        // Send the result back to the controller.
        debug!("Sending combined response to controller");
        self.controller.write_line(&resp_buf).await?;

        info!("AllVersions request completed successfully");
        Ok(())
    }

    async fn process_ferox_request(&mut self, req: FeroxRequest) -> Result<()> {
        match req {
            FeroxRequest::AllVersions => {
                self.handle_all_versions().await?;
                Ok(())
            }
        }
    }

    async fn read_ferox_request<'b>(&'b mut self) -> Result<FeroxRequest> {
        let mut cmd_buf = [0u8; MAX_STRING_SIZE];
        let size = read_until(&mut self.controller, &mut cmd_buf, CMD_PROMPT).await?;
        debug!(
            "Received command: {:?}",
            core::str::from_utf8(&cmd_buf[..size]).unwrap_or("<invalid utf8>")
        );
        match ferox::proto::ascii::from_bytes::<FeroxRequest>(&cmd_buf[..size]) {
            Ok(req) => Ok(req),
            Err(_) => Err(Error::InvalidRequest),
        }
    }

    async fn read_and_process(&mut self) -> Result<()> {
        // TODO(xguo): Keep the input / output buffer here, and reuse them for request / response handling.
        let req = self.read_ferox_request().await?;
        self.process_ferox_request(req).await
    }
}

async fn handle_error<UART, P>(err: Error, w: &mut UartWrapper<UART, P>) -> Result<()>
where
    UART: Read + Write,
    P: PostProcessor,
{
    use core::fmt::Write as FmtWrite;
    let error_num = err as u16;
    let mut s = String::<10>::new();
    write!(s, "0x{:04X}", error_num).map_err(|_| Error::FormatErrorInWriteError)?;
    w.write_line(s.as_str()).await?;
    Ok(())
}

#[embassy_executor::task]
pub async fn run_server(
    controller: BufferedUart<'static, UART4>,
    ctl200: BufferedUart<'static, UART5>,
    smc: BufferedUart<'static, UART7>,
) -> ! {
    let mut server = FeroxServer::new(
        UartWrapper::new(controller, VaPostProcessor),
        UartWrapper::new(ctl200, VaPostProcessor),
        UartWrapper::new(smc, VaPostProcessor),
    );
    loop {
        match server.read_and_process().await {
            Ok(_) => {
                info!("Request processed successfully");
            }
            Err(err) => {
                if let Err(err) = handle_error(err, &mut server.controller).await {
                    error!("Failed to handle error: {}", err);
                }
            }
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    info!("Hello UART Tunnel!");

    static mut TX4_BUF: [u8; 256] = [0; 256];
    static mut RX4_BUF: [u8; 256] = [0; 256];

    let config4 = Config::default();
    #[allow(static_mut_refs)]
    let usart4 = unsafe {
        BufferedUart::new(
            p.UART4,
            Irqs4,
            p.PB8, // RX
            p.PB9, // TX
            &mut TX4_BUF,
            &mut RX4_BUF,
            config4,
        )
        .unwrap()
    };

    static mut TX5_BUF: [u8; 256] = [0; 256];
    static mut RX5_BUF: [u8; 256] = [0; 256];

    let config5 = Config::default();
    #[allow(static_mut_refs)]
    let usart5 = unsafe {
        BufferedUart::new(
            p.UART5,
            Irqs5,
            p.PB5,
            p.PB6,
            &mut TX5_BUF,
            &mut RX5_BUF,
            config5,
        )
        .unwrap()
    };

    static mut TX7_BUF: [u8; 256] = [0; 256];
    static mut RX7_BUF: [u8; 256] = [0; 256];

    let mut config7 = Config::default();
    config7.baudrate = 230400;
    #[allow(static_mut_refs)]
    let usart7 = unsafe {
        BufferedUart::new(
            p.UART7,
            Irqs7,
            p.PF6,
            p.PF7,
            &mut TX7_BUF,
            &mut RX7_BUF,
            config7,
        )
        .unwrap()
    };

    spawner.spawn(run_server(usart4, usart5, usart7)).unwrap();

    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
