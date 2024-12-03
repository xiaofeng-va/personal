use embedded_io::{Read, Write};

/// Driver for CTL200 Laser Controller
///
/// See <https://www.koheron.com/support/user-guides/ctl200/>.
pub struct Ctl200<U>
where
    U: Read + Write + 'static,
{
    _uart: U,
}

impl<U> Ctl200<U>
where
    U: Read + Write + 'static,
{
    pub fn new(uart: U) -> Self {
        Ctl200 { _uart: uart }
    }

    pub fn get(_param: &[u8]) -> Result<Value> {
        todo!()
    }

    pub fn set(_param: &[u8], _value: Value) -> Result<()> {
        todo!()
    }
}

pub enum Value {
    Bool(bool),
    Int(i32),
    Float(f32),
    None,
}

#[derive(Debug)]
pub enum Error {
    // TODO: Add error variants
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Error")
    }
}
impl core::error::Error for Error {}

pub type Result<T> = core::result::Result<T, Error>;
