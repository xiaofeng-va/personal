
pub trait PostProcessor {
    fn post_process<'a>(&self, data: &'a [u8]) -> &'a [u8];
}

// VaPostProcessor should remove the first line of the data.
pub struct VaPostProcessor;

impl PostProcessor for VaPostProcessor {
    fn post_process<'a>(&self, data: &'a [u8]) -> &'a [u8] {
        // Find first newline
        if let Some(pos) = data.iter().position(|&x| x == b'\n') {
            &data[pos + 1..]
        } else {
            &data[..]
        }
    }
}

// DefaultPostProcessor should do nothing.
pub struct DefaultPostProcessor;

impl PostProcessor for DefaultPostProcessor {
    fn post_process<'a>(&self, data: &'a [u8]) -> &'a [u8] {
        data
    }
}
