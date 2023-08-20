use crate::sstable::SSTable;
use anyhow::Result;

pub const SSTABLE_SIZE_LIMIT: usize = 4 * 1024 * 1024; // 4MB
pub struct SSTableBuilder {}

impl SSTableBuilder {
    pub fn new() -> Self {
        todo!()
    }
    // TODO(summerxwu): Maybe need a return value to indicate the result
    pub fn add(&mut self, key: &[u8], value: &[u8]) {
        todo!()
    }
    pub fn build(&self) -> Result<SSTable> {
        todo!()
    }
    fn evaluate_sstable_size(&self) -> usize {
        todo!()
    }
}
