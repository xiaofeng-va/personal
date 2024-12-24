use core::str::from_utf8;

use heapless::String;
use postcard::{ser_flavors::Flavor, Error as PostcardError, Result as PostCardResult};

#[derive(Default)]
pub struct FeroxString<const N: usize> {
    str: String<N>,
}

impl<const N: usize> FeroxString<N> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn release(self) -> PostCardResult<String<N>> {
        Ok(self.str)
    }
}

impl<const N: usize> Flavor for FeroxString<N> {
    type Output = String<N>;

    fn try_extend(&mut self, data: &[u8]) -> PostCardResult<()> {
        self.str
            .push_str(from_utf8(data).map_err(|_| PostcardError::SerializeBufferFull)?)
            .map_err(|_| PostcardError::SerializeBufferFull)
    }

    fn try_push(&mut self, data: u8) -> PostCardResult<()> {
        self.str
            .push(data as char)
            .map_err(|_| PostcardError::SerializeBufferFull)
    }

    fn finalize(self) -> PostCardResult<Self::Output> {
        // Should not call this.
        todo!()
    }
}
