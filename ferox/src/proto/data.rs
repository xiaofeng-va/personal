use heapless::{String, Vec};
use serde::{Deserialize, Serialize};

use crate::MAX_STRING_SIZE;

use super::errors::Error;


#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FeroxProto {
    FeroxRequest(FeroxRequestType),
    FeroxResponse(FeroxResponseType),
    Ctl200Request(Ctl200RequestType),
    Ctl200Response(Ctl200ResponseType),

    Quit,
    Error(Error),
    Unknown,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FeroxRequestType {
    FeroxPing,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FeroxResponseType {
    FeroxPong,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctl200RequestType {
    Ctl200Version,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Ctl200ResponseType {
    Ctl200Version(String<MAX_STRING_SIZE>),
}

#[cfg(feature = "defmt")]
impl defmt::Format for Ctl200ResponseType {
    fn format(&self, f: defmt::Formatter) {
        // constants only to save memory.
        match self {
            Ctl200ResponseType::Ctl200Version(_) => {
                defmt::write!(f, "Ctl200Version");
            }
        }
    }
}
