use core::str::FromStr;

use embedded_io_async::{Read, Write};
use ferox::{
    error,
    proto::{
        data::{
            Ctl200RequestType, Ctl200ResponseType, FeroxProto, FeroxRequestType, FeroxResponseType,
        },
        errors::{Error, Result},
    },
    MAX_STRING_SIZE,
};
use heapless::{String, Vec};

use crate::processor::Processor;

impl<U1, U2> Processor<U1, U2>
where
    U1: Write + 'static,
    U2: Read + Write + 'static,
{
    pub async fn handle_request(&mut self, req: FeroxProto) -> Result<FeroxProto> {
        match req {
            FeroxProto::FeroxRequest(ferox_req) => Ok(FeroxProto::FeroxResponse(
                self.handle_ferox_request(&ferox_req).await?,
            )),
            FeroxProto::Ctl200Request(ctl200_req) => Ok(FeroxProto::Ctl200Response(
                self.handle_ctl200_request(&ctl200_req).await?,
            )),
            FeroxProto::FeroxResponse(_ferox_response_type) => todo!(),
            FeroxProto::Ctl200Response(_ctl200_response_type) => todo!(),
            FeroxProto::Error(_error) => todo!(),
            FeroxProto::Unknown => todo!(),
            FeroxProto::Quit => {
                error!("Quit is server command, should not be here");
                Err(Error::X86Quit)
            }
        }
    }

    pub async fn handle_ferox_request(
        &mut self,
        ferox_req: &FeroxRequestType,
    ) -> Result<FeroxResponseType> {
        match ferox_req {
            FeroxRequestType::FeroxPing => Ok(FeroxResponseType::FeroxPong),
        }
    }

    pub async fn handle_ctl200_request(
        &mut self,
        ctl200_req: &Ctl200RequestType,
    ) -> Result<Ctl200ResponseType> {
        let mut ctl200 = self.ctl200_provider.get_ctl200();
        // TODO(xguo): Add CTL200 logic here, especially the lifetime of Ctl200.
        match ctl200_req {
            Ctl200RequestType::Ctl200Version => {
                let version = ctl200.version().await?;
                let ver_vec = Vec::<u8, MAX_STRING_SIZE>::from_slice(version).map_err(|_| Error::InvalidFirmwareVersion)?;
                let ver_str = String::from_utf8(ver_vec).map_err(|_| Error::InvalidFirmwareVersion)?;
                Ok(Ctl200ResponseType::Ctl200Version(ver_str))
            }
        }
    }
}
