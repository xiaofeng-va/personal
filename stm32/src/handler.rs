use ferox::proto::data::{Ctl200RequestType, Ctl200ResponseType, FeroxProto, FeroxRequestType, FeroxResponseType};
use ferox::proto::errors::Result;
use ferox::{_warn, MAX_STRING_SIZE};
use heapless::String;

pub async fn handle_request(req: FeroxProto) -> Result<FeroxProto> {
    match req {
        FeroxProto::FeroxRequest(ferox_req) => {
            Ok(FeroxProto::FeroxResponse(handle_ferox_request(&ferox_req).await?))
        }
        FeroxProto::Ctl200Request(ctl200_req) => {
            Ok(FeroxProto::Ctl200Response(handle_ctl200_request(&ctl200_req).await?))
        }
        FeroxProto::FeroxResponse(_ferox_response_type) => todo!(),
        FeroxProto::Ctl200Response(_ctl200_response_type) => todo!(),
        FeroxProto::Error(_error) => todo!(),
        FeroxProto::Unknown => todo!(),
    }
}

pub async fn handle_ferox_request(ferox_req: &FeroxRequestType) -> Result<FeroxResponseType> {
    match ferox_req {
        FeroxRequestType::FeroxPing => Ok(FeroxResponseType::FeroxPong),
    }
}

pub async fn handle_ctl200_request(ctl200_req: &Ctl200RequestType) -> Result<Ctl200ResponseType> {
    // TODO(xguo): Add CTL200 logic here, especially the lifetime of Ctl200.
    match ctl200_req {
        Ctl200RequestType::Ctl200Version => Ok(Ctl200ResponseType::Ctl200Version(
            String::<MAX_STRING_SIZE>::try_from("0.1.0").unwrap())),
    }
}