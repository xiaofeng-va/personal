// Due to legacy issues, postcard uses heapless version 0.7.
// Therefore, we cannot use the Vec provided by heapless internally.
// As a workaround, we copy the necessary code here. (xguo)

////////////////////////////////////////
// FeroxVec
////////////////////////////////////////

use heapless::Vec;
use postcard::{ser_flavors::Flavor, Error as PostcardError, Result as PostCardResult};

/// The `FeroxVec` flavor is a wrapper type around a `heapless::Vec`.
/// This is a stack allocated data structure, with a fixed maximum size
/// and variable amount of contents.
#[derive(Default)]
pub struct FeroxVec<const B: usize> {
    /// the contained data buffer
    vec: Vec<u8, B>,
}

impl<const B: usize> FeroxVec<B> {
    /// Create a new, currently empty, [`heapless::Vec`] to be used for storing serialized
    /// output data.
    pub fn new() -> Self {
        Self::default()
    }

    // This is similar to finalize(), but due to access restrictions, we need to construct it ourselves.
    pub fn release(self) -> PostCardResult<Vec<u8, B>> {
        Ok(self.vec)
    }
}

impl<const B: usize> Flavor for FeroxVec<B> {
    type Output = Vec<u8, B>;

    #[inline(always)]
    fn try_extend(&mut self, data: &[u8]) -> PostCardResult<()> {
        self.vec
            .extend_from_slice(data)
            .map_err(|_| postcard::Error::SerializeBufferFull)
    }

    #[inline(always)]
    fn try_push(&mut self, data: u8) -> PostCardResult<()> {
        self.vec
            .push(data)
            .map_err(|_| PostcardError::SerializeBufferFull)
    }

    fn finalize(self) -> PostCardResult<Vec<u8, B>> {
        Ok(self.vec)
    }
}
