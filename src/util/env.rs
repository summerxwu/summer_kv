use anyhow::Result;
use bytes::Bytes;
use std::fs;

pub struct FileObject {
    file_handler: fs::File,
    /// size of current file
    size: u64,
}
impl FileObject {
    /// Create a new FileObject by a given file name.
    /// Failed if file exists.
    pub fn create(path: &str) -> Result<Self> {
        todo!()
    }

    /// Open a new FileObject by a given file name.
    /// Failed if file not exists.
    pub fn open(path: &str) -> Result<Self> {
        todo!()
    }

    /// Read content from file from `offset` by `length` long
    pub fn read(&self, offset: usize, length: usize) -> Result<Bytes> {
        todo!()
    }

    /// Read Last length bytes of file content
    pub fn read_last_of(&self, length: usize) -> Result<Bytes> {
        todo!()
    }

    /// Write Content to file
    pub fn write(&mut self, buf: &[u8]) -> Result<()> {
        todo!()
    }
    // Do fsync(), flush data to disk
    pub fn sync() -> Result<()> {
        todo!()
    }

    // Return the approximate size of current file
    pub fn size(&self) -> u64 {
        self.size
    }
}
pub fn sstfile_path(seq: u8) -> String {
    todo!()
}
