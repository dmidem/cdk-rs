//! APIs to manage stable memory.
//!
//! You can check the [Internet Computer Specification](https://smartcontracts.org/docs/interface-spec/index.html#system-api-stable-memory)
//! for a in-depth explanation of stable memory.

use std::{error, fmt};

/// A possible error value when dealing with stable memory.
#[derive(Debug)]
pub enum StableMemoryError {
    /// No more stable memory could be allocated.
    OutOfMemory,
    /// Attempted to read more stable memory than had been allocated.
    OutOfBounds,
}

impl fmt::Display for StableMemoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::OutOfMemory => f.write_str("Out of memory"),
            Self::OutOfBounds => f.write_str("Read exceeds allocated memory"),
        }
    }
}

impl error::Error for StableMemoryError {}

/// A trait defining the stable memory API which each canister running on the IC can make use of
pub trait StableMemory {
    /// Gets current size of the stable memory (in WASM pages).
    fn stable_size(&self) -> u32;

    /// Similar to `stable_size` but with support for 64-bit addressed memory.
    fn stable64_size(&self) -> u64;

    /// Attempts to grow the stable memory by `new_pages` (added pages).
    ///
    /// Returns an error if it wasn't possible. Otherwise, returns the previous
    /// size that was reserved.
    ///
    /// *Note*: Pages are 64KiB in WASM.
    fn stable_grow(&self, new_pages: u32) -> Result<u32, StableMemoryError>;

    /// Similar to `stable_grow` but with support for 64-bit addressed memory.
    fn stable64_grow(&self, new_pages: u64) -> Result<u64, StableMemoryError>;

    /// Writes data to the stable memory location specified by an offset.
    ///
    /// Warning - this will panic if `offset + buf.len()` exceeds the current size of stable memory.
    /// Use `stable_grow` to request more stable memory if needed.
    fn stable_write(&self, offset: u32, buf: &[u8]);

    /// Similar to `stable_write` but with support for 64-bit addressed memory.
    fn stable64_write(&self, offset: u64, buf: &[u8]);

    /// Reads data from the stable memory location specified by an offset.
    fn stable_read(&self, offset: u32, buf: &mut [u8]);

    /// Similar to `stable_read` but with support for 64-bit addressed memory.
    fn stable64_read(&self, offset: u64, buf: &mut [u8]);
}
