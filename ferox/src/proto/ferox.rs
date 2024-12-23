use serde::{Deserialize, Serialize};

use crate::proto::error;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum FeroxRequest<'a> {
    #[serde(rename = "fail")]
    Failure { error: error::Error },

    #[serde(rename = "ver")]
    Version,

    #[serde(rename = "another")]
    Another(AnotherEnum),

    #[serde(rename = "smc")]
    SmcForward { data: &'a [u8] }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum AnotherEnum {
    #[serde(rename = "ver")]
    Version,
}