use std::io;

use super::{CanisterStableMemory, StableIO, StableMemory, StableMemoryError};

// A reader to the stable memory.
///
/// Keeps an offset and reads off stable memory consecutively.
pub struct StableReader<M: StableMemory = CanisterStableMemory>(StableIO<M, u32>);

impl Default for StableReader {
    fn default() -> Self {
        Self(StableIO::default())
    }
}

impl<M: StableMemory> StableReader<M> {
    /// Creates a new `StableReader` which reads from the selected memory
    #[inline]
    pub fn with_memory(memory: M, offset: usize) -> Self {
        Self(StableIO::<M, u32>::with_memory(memory, offset as u32))
    }

    /// Returns the offset of the reader
    #[inline]
    pub fn offset(&self) -> usize {
        self.0.offset() as usize
    }

    /// Reads data from the stable memory location specified by an offset.
    ///
    /// Note:
    /// The stable memory size is cached on creation of the StableReader.
    /// Therefore, in following scenario, it will get an `OutOfBounds` error:
    /// 1. Create a StableReader
    /// 2. Write some data to the stable memory which causes it grow
    /// 3. call `read()` to read the newly written bytes
    #[inline]
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, StableMemoryError> {
        self.0.read(buf)
    }
}

impl<M: StableMemory> io::Read for StableReader<M> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        io::Read::read(&mut self.0, buf)
    }
}

impl<M: StableMemory> io::Seek for StableReader<M> {
    #[inline]
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        io::Seek::seek(&mut self.0, pos)
    }
}

impl<M: StableMemory> From<StableIO<M>> for StableReader<M> {
    fn from(io: StableIO<M>) -> Self {
        Self(io)
    }
}

/// A reader to the stable memory which reads bytes a chunk at a time as each chunk is required.
pub struct BufferedStableReader<M: StableMemory = CanisterStableMemory> {
    inner: io::BufReader<StableReader<M>>,
}

impl BufferedStableReader {
    /// Creates a new `BufferedStableReader`
    pub fn new(buffer_size: usize) -> BufferedStableReader {
        BufferedStableReader::with_reader(buffer_size, StableReader::default())
    }
}

impl<M: StableMemory> BufferedStableReader<M> {
    /// Creates a new `BufferedStableReader` which reads from the selected memory
    pub fn with_reader(buffer_size: usize, reader: StableReader<M>) -> BufferedStableReader<M> {
        BufferedStableReader {
            inner: io::BufReader::with_capacity(buffer_size, reader),
        }
    }

    /// Returns the offset of the reader
    pub fn offset(&self) -> usize {
        self.inner.get_ref().offset()
    }
}

impl<M: StableMemory> io::Read for BufferedStableReader<M> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl<M: StableMemory> io::Seek for BufferedStableReader<M> {
    #[inline]
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        io::Seek::seek(&mut self.inner, pos)
    }
}
