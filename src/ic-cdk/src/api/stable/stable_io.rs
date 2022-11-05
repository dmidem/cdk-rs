use std::io;

use super::{private, CanisterStableMemory, StableMemory, StableMemoryError};

pub(super) const WASM_PAGE_SIZE_IN_BYTES: usize = 64 * 1024; // 64KB

/// Performs generic IO (read, write, and seek) on stable memory.
///
/// Warning: When using write functionality, this will overwrite any existing
/// data in stable memory as it writes, so ensure you set the `offset` value
/// accordingly if you wish to preserve existing data.
///
/// Will attempt to grow the memory as it writes,
/// and keep offsets and total capacity.

pub struct StableIO<M: StableMemory = CanisterStableMemory, A: private::AddressSize = u32> {
    /// The offset of the next write.
    offset: A,

    /// The capacity, in pages.
    capacity: A,

    /// The stable memory to write data to.
    memory: M,
}

impl Default for StableIO {
    fn default() -> Self {
        Self::with_memory(CanisterStableMemory::default(), 0)
    }
}

// Helper macro to implement StableIO for both 32-bit and 64-bit.
//
// We use a macro here since capturing all the traits required to add and manipulate memory
// addresses with generics becomes cumbersome.
macro_rules! impl_stable_io {
    ($address:ty) => {
        impl<M: private::StableMemory_<$address> + StableMemory> StableIO<M, $address> {
            /// Creates a new `StableIO` which writes to the selected memory
            pub fn with_memory(memory: M, offset: $address) -> Self {
                let capacity = memory.stable_size_();

                Self {
                    offset,
                    capacity,
                    memory,
                }
            }

            /// Returns the offset of the writer
            pub fn offset(&self) -> $address {
                self.offset
            }

            /// Attempts to grow the memory by adding new pages.
            pub fn grow(&mut self, new_pages: $address) -> Result<(), StableMemoryError> {
                let old_page_count = self.memory.stable_grow_(new_pages)?;
                self.capacity = old_page_count + new_pages;
                Ok(())
            }

            /// Writes a byte slice to the buffer.
            ///
            /// The only condition where this will
            /// error out is if it cannot grow the memory.
            pub fn write(&mut self, buf: &[u8]) -> Result<usize, StableMemoryError> {
                let required_capacity_bytes = self.offset + buf.len() as $address;
                let required_capacity_pages =
                    ((required_capacity_bytes + WASM_PAGE_SIZE_IN_BYTES as $address - 1)
                        / WASM_PAGE_SIZE_IN_BYTES as $address);
                let current_pages = self.capacity;
                let additional_pages_required =
                    required_capacity_pages.saturating_sub(current_pages);

                if additional_pages_required > 0 {
                    self.grow(additional_pages_required)?;
                }

                self.memory.stable_write_(self.offset, buf);
                self.offset += buf.len() as $address;
                Ok(buf.len())
            }

            /// Reads data from the stable memory location specified by an offset.
            ///
            /// Note:
            /// The stable memory size is cached on creation of the StableReader.
            /// Therefore, in following scenario, it will get an `OutOfBounds` error:
            /// 1. Create a StableReader
            /// 2. Write some data to the stable memory which causes it grow
            /// 3. call `read()` to read the newly written bytes
            pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, StableMemoryError> {
                let capacity_bytes = self.capacity * WASM_PAGE_SIZE_IN_BYTES as $address;
                let read_buf = if buf.len() as $address + self.offset > capacity_bytes {
                    if self.offset < capacity_bytes {
                        &mut buf[..(capacity_bytes - self.offset) as usize]
                    } else {
                        return Err(StableMemoryError::OutOfBounds);
                    }
                } else {
                    buf
                };
                self.memory.stable_read_(self.offset, read_buf);
                self.offset += read_buf.len() as $address;
                Ok(read_buf.len())
            }

            // Helper used to implement io::Seek
            fn seek(&mut self, offset: io::SeekFrom) -> io::Result<u64> {
                self.offset = match offset {
                    io::SeekFrom::Start(offset) => offset as $address,
                    io::SeekFrom::End(offset) => {
                        ((self.capacity * WASM_PAGE_SIZE_IN_BYTES as $address) as i64 + offset)
                            as $address
                    }
                    io::SeekFrom::Current(offset) => (self.offset as i64 + offset) as $address,
                };

                Ok(self.offset as u64)
            }
        }

        impl<M: private::StableMemory_<$address> + StableMemory> io::Write
            for StableIO<M, $address>
        {
            fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
                self.write(buf)
                    .map_err(|e| io::Error::new(io::ErrorKind::OutOfMemory, e))
            }

            fn flush(&mut self) -> Result<(), io::Error> {
                // Noop.
                Ok(())
            }
        }

        impl<M: private::StableMemory_<$address> + StableMemory> io::Read
            for StableIO<M, $address>
        {
            fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
                Self::read(self, buf).or(Ok(0)) // Read defines EOF to be success
            }
        }

        impl<M: private::StableMemory_<$address> + StableMemory> io::Seek
            for StableIO<M, $address>
        {
            fn seek(&mut self, offset: io::SeekFrom) -> io::Result<u64> {
                self.seek(offset)
            }
        }
    };
}

impl_stable_io!(u32);
impl_stable_io!(u64);
