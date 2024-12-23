use serde::{Deserialize, Serialize};

use crate::proto::error;

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub enum FeroxRequest<'a> {
    Failure { error: error::Error },

    Version,
    Another(AnotherEnum),

    SmcForward { data: &'a [u8] }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum AnotherEnum {
    Version,
}