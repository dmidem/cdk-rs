use std::io;

use super::{CanisterStableMemory, StableIO, StableMemory, StableMemoryError};

/// A writer to the stable memory.
///
/// Warning: This will overwrite any existing data in stable memory as it writes, so ensure you set
/// the `offset` value accordingly if you wish to preserve existing data.
///
/// Will attempt to grow the memory as it writes,
/// and keep offsets and total capacity.
pub struct StableWriter<M: StableMemory = CanisterStableMemory>(StableIO<M, u32>);

impl Default for StableWriter {
    #[inline]
    fn default() -> Self {
        Self(StableIO::default())
    }
}

impl<M: StableMemory> StableWriter<M> {
    /// Creates a new `StableWriter` which writes to the selected memory
    #[inline]
    pub fn with_memory(memory: M, offset: usize) -> Self {
        Self(StableIO::<M, u32>::with_memory(memory, offset as u32))
    }

    /// Returns the offset of the writer
    #[inline]
    pub fn offset(&self) -> usize {
        self.0.offset() as usize
    }

    /// Attempts to grow the memory by adding new pages.
    #[inline]
    pub fn grow(&mut self, new_pages: u32) -> Result<(), StableMemoryError> {
        self.0.grow(new_pages)
    }

    /// Writes a byte slice to the buffer.
    ///
    /// The only condition where this will
    /// error out is if it cannot grow the memory.
    #[inline]
    pub fn write(&mut self, buf: &[u8]) -> Result<usize, StableMemoryError> {
        self.0.write(buf)
    }
}

impl<M: StableMemory> io::Write for StableWriter<M> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        io::Write::write(&mut self.0, buf)
    }

    #[inline]
    fn flush(&mut self) -> Result<(), io::Error> {
        io::Write::flush(&mut self.0)
    }
}

impl<M: StableMemory> io::Seek for StableWriter<M> {
    #[inline]
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        io::Seek::seek(&mut self.0, pos)
    }
}

impl<M: StableMemory> From<StableIO<M>> for StableWriter<M> {
    fn from(io: StableIO<M>) -> Self {
        Self(io)
    }
}

/// A writer to the stable memory which first writes the bytes to an in memory buffer and flushes
/// the buffer to stable memory each time it becomes full.
///
/// Warning: This will overwrite any existing data in stable memory as it writes, so ensure you set
/// the `offset` value accordingly if you wish to preserve existing data.
///
/// Note: Each call to grow or write to stable memory is a relatively expensive operation, so pick a
/// buffer size large enough to avoid excessive calls to stable memory.
pub struct BufferedStableWriter<M: StableMemory = CanisterStableMemory> {
    inner: io::BufWriter<StableWriter<M>>,
}

impl BufferedStableWriter {
    /// Creates a new `BufferedStableWriter`
    pub fn new(buffer_size: usize) -> BufferedStableWriter {
        BufferedStableWriter::with_writer(buffer_size, StableWriter::default())
    }
}

impl<M: StableMemory> BufferedStableWriter<M> {
    /// Creates a new `BufferedStableWriter` which writes to the selected memory
    pub fn with_writer(buffer_size: usize, writer: StableWriter<M>) -> BufferedStableWriter<M> {
        BufferedStableWriter {
            inner: io::BufWriter::with_capacity(buffer_size, writer),
        }
    }

    /// Returns the offset of the writer
    pub fn offset(&self) -> usize {
        self.inner.get_ref().offset()
    }
}

impl<M: StableMemory> io::Write for BufferedStableWriter<M> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

impl<M: StableMemory> io::Seek for BufferedStableWriter<M> {
    #[inline]
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        io::Seek::seek(&mut self.inner, pos)
    }
}
