#![no_std]

pub mod drivers;
pub mod proto;
pub mod uart;

pub const MAX_STRING_SIZE: usize = 256;

#[cfg(test)]
mod testing {
    pub mod helpers;
}

// TODO(xguo): Add coverage in github workflow for the crate.
