use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, channel::Channel};
use embassy_time::{Duration, Timer};
use embedded_io_async::{Read, Write};
use ferox::{
    debug,
    drivers::koheron::ctl200::Ctl200Provider,
    error,
    proto::{
        data::FeroxProto,
        errors::{Error, Result},
    },
};

// TODO(xguo): Rename CHANNEL, too generic.
pub static CHANNEL: Channel<ThreadModeRawMutex, [u8; 1], 1> = Channel::new();

pub struct Processor<U1, U2>
where
    U1: Write + 'static,
    U2: Read + Write + 'static,
{
    tx: U1,
    pub(crate) ctl200_provider: Ctl200Provider<U2>,
}

impl<U1, U2> Processor<U1, U2>
where
    U1: Write + 'static,
    U2: Read + Write + 'static,
{
    pub fn new(tx: U1, ctl200_uart: U2) -> Self {
        Processor {
            tx,
            ctl200_provider: Ctl200Provider::new(ctl200_uart),
        }
    }

    async fn recv_data_with_size(
        &mut self,
        size: usize,
        buf: &'_ mut [u8],
        timeout_ms: u64,
    ) -> Result<()> {
        for byte in &mut buf[..size] {
            let result = embassy_futures::select::select(
                CHANNEL.receive(),
                Timer::after(Duration::from_millis(timeout_ms)),
            )
            .await;

            match result {
                embassy_futures::select::Either::First(data) => *byte = data[0],
                embassy_futures::select::Either::Second(_) => {
                    error!("Timeout while receiving data");
                    return Err(Error::TimeoutError);
                }
            }
        }
        Ok(())
    }

    pub async fn process_message(&mut self) -> Result<()> {
        debug!("Starting to process message");

        // Read the size byte
        let size = CHANNEL.receive().await[0] as usize;
        debug!("Received size: {}", size);

        // Read the content based on the size
        let mut content_buf = [0u8; 256];
        self.recv_data_with_size(size, &mut content_buf, 1_000)
            .await?;
        let recv_data = &content_buf[..size];
        debug!("Received content: {:?}", recv_data);

        let req = postcard::from_bytes::<FeroxProto>(&recv_data).map_err(|_| {
            error!("Failed to deserialize request");
            Error::PostcardDeserializeError
        })?;
        debug!("Deserialized request: {:?}", req);

        let resp = self.handle_request(req).await?;
        debug!("Processed request, response: {:?}", resp);

        let mut buf = [0u8; 256];
        let resp_bytes = postcard::to_slice(&resp, &mut buf).map_err(|_| {
            error!("Failed to serialize response");
            Error::PostcardSerializeError
        })?;
        let resp_size = resp_bytes.len() as u8;
        debug!("Serialized response size: {}", resp_size);

        self.tx.write_all(&[resp_size]).await.map_err(|_| {
            error!("Failed to write response size");
            Error::WriteError
        })?;
        self.tx.write_all(&resp_bytes).await.map_err(|_| {
            error!("Failed to write response bytes");
            Error::WriteError
        })?;
        debug!("Response sent successfully");

        Ok(())
    }
}
