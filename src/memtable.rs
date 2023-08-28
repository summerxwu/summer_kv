use crate::util::env::FileObject;
use anyhow::Result;
use bytes::Bytes;

mod logger;

pub struct MemTable {}

impl MemTable {
    fn new() -> Self {
        todo!()
    }
    fn put(&mut self, key: &[u8], value: &[u8]) -> Result<()> {
        todo!()
    }
    fn get(&self, key: &[u8]) -> Result<Bytes> {
        todo!()
    }
    fn delete(&mut self, key: &[u8]) -> Result<()> {
        todo!()
    }
    fn recover(file: &FileObject) -> Self {
        todo!()
    }
}
