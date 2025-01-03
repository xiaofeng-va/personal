pub mod post_processor;

use core::cmp::min;

use defmt_or_log::debug;
use embassy_time::Duration;
use embedded_io_async::{Read, Write};
use post_processor::PostProcessor;

use crate::proto::{error::Error as FeroxError, Result as FeroxResult};

// TODO(xguo): test the function.
pub async fn read_until<R: Read>(
    reader: &mut R,
    buf: &mut [u8],
    terminator: &[u8],
) -> FeroxResult<usize> {
    const TEMP_SIZE: usize = 64;
    let buf_len = buf.len();
    let mut temp = [0u8; TEMP_SIZE];
    let mut pos = 0;

    while pos + terminator.len() <= buf.len() {
        let chunk_size = min(TEMP_SIZE, buf_len - pos);
        let sz = reader
            .read(&mut temp[..chunk_size])
            .await
            .map_err(|_| FeroxError::UartReadError)?;
        if sz == 0 {
            return Err(FeroxError::UartReadError);
        }
        debug!(
            "Read {} bytes: {:?}",
            sz,
            core::str::from_utf8(&temp[..sz]).unwrap_or("<invalid utf8>")
        );
        buf[pos..pos + sz].copy_from_slice(&temp[..sz]);
        pos += sz;

        if pos >= terminator.len() {
            let start_idx = pos - terminator.len();
            if &buf[start_idx..pos] == terminator {
                return Ok(pos - terminator.len());
            }
        }
    }

    Err(FeroxError::BufferOverflow)
}

pub struct UartWrapper<UART, P> {
    uart: UART,
    post_processor: P,
}

impl<UART, P> UartWrapper<UART, P>
where
    UART: Read + Write,
    P: PostProcessor,
{
    pub fn new(uart: UART, post_processor: P) -> Self {
        Self {
            uart,
            post_processor,
        }
    }

    async fn try_once(
        &mut self,
        request: &[u8],
        response_buf: &mut [u8],
        terminator: &[u8],
        timeout: Duration,
    ) -> FeroxResult<usize> {
        self.uart
            .write_all(request)
            .await
            .map_err(|_| FeroxError::UartWriteErrorInTryOnce)?;
        self.uart
            .write_all(b"\r\n")
            .await
            .map_err(|_| FeroxError::UartWriteErrorInTryOnce)?;
        self.uart
            .flush()
            .await
            .map_err(|_| FeroxError::UartFlushError)?;

        embassy_time::with_timeout(
            timeout,
            read_until(&mut self.uart, response_buf, terminator),
        )
        .await
        .map_err(|_| FeroxError::UartRequestTimeout)?
    }

    pub async fn query_with_pattern<'a>(
        &mut self,
        request: &[u8],
        terminator: &[u8],
        response_buf: &'a mut [u8],
        timeout: Duration,
        max_retries: i32,
    ) -> FeroxResult<&'a [u8]> {
        for attempt in 1..=max_retries {
            match self
                .try_once(request, response_buf, terminator, timeout)
                .await
            {
                Ok(size) => {
                    debug!("Query succeeded on attempt {}", attempt);
                    let processed_data = self.post_processor.post_process(&response_buf[..size]);
                    return Ok(processed_data);
                }
                Err(e) => {
                    debug!("Error during attempt {}: {:?}", attempt, e);
                    if attempt == max_retries {
                        debug!("Max retries reached. Failing with error: {:?}", e);
                        return Err(e);
                    }
                }
            }
        }
        Err(FeroxError::UartRequestTimeout) // 理论上不会到这里
    }

    pub async fn write_line(&mut self, line: &str) -> FeroxResult<()> {
        self.uart
            .write_all(line.as_bytes())
            .await
            .map_err(|_| FeroxError::UartWriteErrorInWriteLine)?;
        self.uart
            .write_all(b"\r\n")
            .await
            .map_err(|_| FeroxError::UartWriteErrorInWriteLine)?;
        self.uart
            .flush()
            .await
            .map_err(|_| FeroxError::UartFlushError)?;
        Ok(())
    }
}

impl<UART, P> embedded_io_async::ErrorType for UartWrapper<UART, P>
where
    UART: Read + Write,
    P: PostProcessor,
{
    type Error = UART::Error;
}

impl<UART, P> embedded_io_async::Read for UartWrapper<UART, P>
where
    UART: Read + Write,
    P: PostProcessor,
{
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.uart.read(buf).await
    }
}
