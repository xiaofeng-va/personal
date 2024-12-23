pub mod ascii;
pub mod error;
pub mod ferox;

pub type Result<T> = core::result::Result<T, error::Error>;
