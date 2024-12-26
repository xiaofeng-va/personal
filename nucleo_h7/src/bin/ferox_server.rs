#![no_std]
#![no_main]

use {defmt_rtt as _, panic_probe as _};

use core::cmp::min;
use defmt::{expect, info};
use embassy_stm32::peripherals::{UART4, UART5, UART7};
use embassy_stm32::usart::{
    BufferedUart, BufferedUartRx, BufferedUartTx, Config, Uart,
};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use embassy_time::{Duration, Timer};

use embassy_executor::Spawner;
use embedded_io_async::{Read, Write};
use ferox::proto::{
    ascii::{from_bytes, to_string},
    error::Error,
    ferox::{Ctl200Request, FeroxRequest, SmcRequest},
    Result,
};
use heapless::{String, Vec};

bind_interrupts!(struct Irqs4 {
    UART4 => usart::BufferedInterruptHandler<peripherals::UART4>;
});

bind_interrupts!(struct Irqs5 {
    UART5 => usart::BufferedInterruptHandler<peripherals::UART5>;
});

bind_interrupts!(struct Irqs7 {
    UART7 => usart::BufferedInterruptHandler<peripherals::UART7>;
});

// 根据你的实际需求来决定最大 buffer 尺寸
pub const MAX_STRING_SIZE: usize = 256;

// 这个结束符仅在演示：因为我们要从 UART4 读取到问号（或 CRLF）就开始解析
pub const CMD_PROMPT: &[u8] = b"\r\n";

// 结束符举例
pub const CTL200_END: &[u8] = b"\r\n<<";
pub const SMC_END: &[u8] = b"\r\n\r\n";

// TODO(xguo): test the function.
async fn read_until<R: Read>(reader: &mut R, buf: &mut [u8], terminator: &[u8]) -> Result<usize> {
    const TEMP_SIZE: usize = 64;
    let buf_len = buf.len();
    let mut temp = [0u8; TEMP_SIZE];
    let mut pos = 0;

    while pos + terminator.len() <= buf.len() {
        let chunk_size = min(TEMP_SIZE, buf_len - pos);
        let sz = reader.read(&mut temp[..chunk_size]).await.map_err(|_| Error::ReadError)?;
        if sz == 0 {
            return Err(Error::ReadError);
        }

        // 拷贝到 buf
        buf[pos..pos + sz].copy_from_slice(&temp[..sz]);
        pos += sz;

        if pos >= terminator.len() {
            let start_idx = pos - terminator.len();
            if &buf[start_idx..pos] == terminator {
                return Ok(pos - terminator.len());
            }
        }
    }

    Err(Error::BufferOverflow)
}

/// 把对 UART 的读写包装进一个通用 struct，方便重用你之前的 query() / read_until() 逻辑。
pub struct UartWrapper<UART> {
    uart: UART,
}

impl<UART> UartWrapper<UART>
where
    UART: Read + Write,
{
    pub fn new(uart: UART) -> Self {
        Self { uart }
    }

    pub async fn query_with_pattern(
        &mut self,
        request: &str,
        terminator: &[u8],
        response_buf: &'_ mut [u8],
    ) -> Result<usize> {
        self.write_line(request).await?;
        read_until(&mut self.uart, response_buf, terminator).await
    }

    async fn write_line(&mut self, line: &str) -> Result<()> {
        self.uart
            .write_all(line.as_bytes())
            .await
            .map_err(|_| Error::WriteError)?;
        self.uart
            .write_all(b"\r\n")
            .await
            .map_err(|_| Error::WriteError)?;
        self.uart.flush().await.map_err(|_| Error::FlushError)?;
        Ok(())
    }
}

pub struct FeroxServer<U0, U1, U2> {
    /// UART4 is used to receive/send commands from the host or external devices
    controller: UartWrapper<U0>,
    /// UART5 is used to communicate with the ctl200 device
    ctl200: UartWrapper<U1>,
    /// UART7 is used to communicate with the smc device
    smc: UartWrapper<U2>,
}

impl<U0, U1, U2> FeroxServer<U0, U1, U2>
where
    U0: embedded_io_async::Read + embedded_io_async::Write,
    U1: embedded_io_async::Read + embedded_io_async::Write,
    U2: embedded_io_async::Read + embedded_io_async::Write,
{
    pub fn new(controller: U0, ctl200: U1, smc: U2) -> Self {
        Self {
            controller: UartWrapper::new(controller),
            ctl200: UartWrapper::new(ctl200),
            smc: UartWrapper::new(smc),
        }
    }

    async fn handle_all_versions(&mut self) -> Result<()> {
        let ctl200_req_str = to_string(&Ctl200Request::Version).map_err(|_| Error::WriteError)?;
        let smc_req_str = to_string(&SmcRequest::Version(None)).map_err(|_| Error::WriteError)?;

        // 1. Send to ctl200
        let mut response_buf = [0u8; MAX_STRING_SIZE];
        let ctl_resp_size = self
            .ctl200
            .query_with_pattern(&ctl200_req_str, CTL200_END, &mut response_buf)
            .await?;
        let ctl200_ver =
            from_bytes::<&[u8]>(&response_buf[..ctl_resp_size]).map_err(|_| Error::InvalidResponse)?;

        // 2. Send to SMC.
        // TODO(xguo): You can use async-std::future::join() to send them concurrently.
        let mut smc_response_buf = [0u8; MAX_STRING_SIZE];
        let smc_resp_size = self.smc.query_with_pattern(&smc_req_str, SMC_END, &mut smc_response_buf).await?;
        let smc_ver = from_bytes::<&[u8]>(&smc_response_buf[..smc_resp_size]).map_err(|_| Error::InvalidResponse)?;

        // 3. Assemble the final string
        let mut resp_buf: String<MAX_STRING_SIZE> = String::new();
        {
            use core::fmt::Write;
            write!(
                resp_buf,
                "<ctl200>\r\n{}\r\n<smc>\r\n{}",
                core::str::from_utf8(ctl200_ver).unwrap_or("<invalid>"),
                core::str::from_utf8(smc_ver).unwrap_or("<invalid>"),
            ).map_err(|_| Error::WriteError)?;
        }

        // Send the result back to the controller.
        self.controller.write_line(&resp_buf).await?;

        Ok(())
    }

    async fn process_ferox_request(&mut self, req: FeroxRequest) -> Result<()> {
        match req {
            FeroxRequest::AllVersions => {
                if let Err(e) = self.handle_all_versions().await {
                    let _ = self
                        .controller
                        .write_line("Errors in handle_all_versions()")
                        .await;
                }
                Ok(())
            }
        }
    }

    async fn read_ferox_request<'b>(&'b mut self) -> Result<FeroxRequest> {
        let mut cmd_buf = [0u8; MAX_STRING_SIZE];
        let size = read_until(&mut self.controller.uart, &mut cmd_buf, CMD_PROMPT).await?;
        match ferox::proto::ascii::from_bytes::<FeroxRequest>(&cmd_buf[..size]) {
            Ok(req) => Ok(req),
            Err(_) => Err(Error::InvalidRequest),
        }
    }
}

#[embassy_executor::task]
pub async fn run_server(
    controller: BufferedUart<'static, UART4>,
    ctl200: BufferedUart<'static, UART5>,
    smc: BufferedUart<'static, UART7>,
) -> ! {
    let mut server = FeroxServer::new(controller, ctl200, smc);
    loop {
        match server.read_ferox_request().await {
            Ok(req) => {
                info!("Received request: {:?}", req);
                server.process_ferox_request(req).await;
            }
            Err(_) => {
                let _ = server.controller.write_line("ferox: Unknown command").await;
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
            p.PB8,  // RX
            p.PB9,  // TX
            &mut TX4_BUF,
            &mut RX4_BUF,
            config4,
        )
        .unwrap()
    };
    // let (tx4, rx4) = usart4.split();

    static mut TX5_BUF: [u8; 256] = [0; 256];
    static mut RX5_BUF: [u8; 256] = [0; 256];

    let mut config5 = Config::default();
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
    // 将 UART7 分拆成 (tx7, rx7)
    // let (tx5, rx5) = usart5.split();

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
    // let (tx7, rx7) = usart7.split();

    spawner.spawn(run_server(usart4, usart5, usart7)).unwrap();

    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
