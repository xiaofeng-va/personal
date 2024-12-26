use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FeroxRequest {
    #[serde(rename = "allver")]
    AllVersions,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ctl200Request {
    #[serde(rename = "version")]
    Version,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SmcRequest<'a> {
    #[serde(rename = "bia")]
    Version(Option<&'a [u8]>),
}