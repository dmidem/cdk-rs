use super::{CanisterStableMemory, StableMemory, StableMemoryError};

static CANISTER_STABLE_MEMORY: CanisterStableMemory = CanisterStableMemory {};

/// Gets current size of the stable memory (in WASM pages).
pub fn stable_size() -> u32 {
    CANISTER_STABLE_MEMORY.stable_size()
}

/// Similar to `stable_size` but with support for 64-bit addressed memory.
pub fn stable64_size() -> u64 {
    CANISTER_STABLE_MEMORY.stable64_size()
}

/// Attempts to grow the stable memory by `new_pages` (added pages).
///
/// Returns an error if it wasn't possible. Otherwise, returns the previous
/// size that was reserved.
///
/// *Note*: Pages are 64KiB in WASM.
pub fn stable_grow(new_pages: u32) -> Result<u32, StableMemoryError> {
    CANISTER_STABLE_MEMORY.stable_grow(new_pages)
}

/// Similar to `stable_grow` but with support for 64-bit addressed memory.
pub fn stable64_grow(new_pages: u64) -> Result<u64, StableMemoryError> {
    CANISTER_STABLE_MEMORY.stable64_grow(new_pages)
}

/// Writes data to the stable memory location specified by an offset.
///
/// Warning - this will panic if `offset + buf.len()` exceeds the current size of stable memory.
/// Use `stable_grow` to request more stable memory if needed.
pub fn stable_write(offset: u32, buf: &[u8]) {
    CANISTER_STABLE_MEMORY.stable_write(offset, buf)
}

/// Similar to `stable_write` but with support for 64-bit addressed memory.
pub fn stable64_write(offset: u64, buf: &[u8]) {
    CANISTER_STABLE_MEMORY.stable64_write(offset, buf)
}

/// Reads data from the stable memory location specified by an offset.
pub fn stable_read(offset: u32, buf: &mut [u8]) {
    CANISTER_STABLE_MEMORY.stable_read(offset, buf)
}

/// Similar to `stable_read` but with support for 64-bit addressed memory.
pub fn stable64_read(offset: u64, buf: &mut [u8]) {
    CANISTER_STABLE_MEMORY.stable64_read(offset, buf)
}

/// Returns a copy of the stable memory.
///
/// This will map the whole memory (even if not all of it has been written to).
pub fn stable_bytes() -> Vec<u8> {
    let size = (stable_size() as usize) << 16;
    let mut vec = Vec::with_capacity(size);
    unsafe {
        ic0::stable_read(vec.as_ptr() as i32, 0, size as i32);
        vec.set_len(size);
    }
    vec
}
