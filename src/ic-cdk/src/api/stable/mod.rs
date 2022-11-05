//! APIs to manage stable memory.
//!
//! You can check the [Internet Computer Specification](https://smartcontracts.org/docs/interface-spec/index.html#system-api-stable-memory)
//! for a in-depth explanation of stable memory.

mod canister;
mod canister_static;
mod private;
mod stable_io;
mod stable_memory;
mod stable_reader;
mod stable_writer;

#[cfg(test)]
mod tests;

pub use canister::CanisterStableMemory;

pub use canister_static::{
    stable64_grow, stable64_read, stable64_size, stable64_write, stable_bytes, stable_grow,
    stable_read, stable_size, stable_write,
};

pub use stable_memory::{StableMemory, StableMemoryError};

pub use stable_reader::{BufferedStableReader, StableReader};

pub use stable_writer::{BufferedStableWriter, StableWriter};

use stable_io::StableIO;

#[cfg(test)]
use stable_io::WASM_PAGE_SIZE_IN_BYTES;
