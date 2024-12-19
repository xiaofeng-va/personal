use ferox::proto::data::{Ctl200RequestType, Ctl200ResponseType, FeroxRequestType, FeroxResponseType};
use ferox::proto::errors::Result;
use ferox::MAX_STRING_SIZE;
use heapless::String;

pub fn handle_ferox_request(ferox_req: &FeroxRequestType) -> Result<FeroxResponseType> {
    match ferox_req {
        FeroxRequestType::FeroxPing => Ok(FeroxResponseType::FeroxPong),
    }
}

pub fn handle_ctl200_request(ctl200_req: &Ctl200RequestType) -> Result<Ctl200ResponseType> {
    // TODO(xguo): Add CTL200 logic here, especially the lifetime of Ctl200.
    match ctl200_req {
        Ctl200RequestType::Ctl200Version => Ok(Ctl200ResponseType::Ctl200Version(
            String::<MAX_STRING_SIZE>::try_from("0.1.0").unwrap())),
    }
}