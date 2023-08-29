use crate::memtable::logger::{LogRecordsBuilder, MemTableLogger, OperationType};
use crate::util::env::{get_global_sequence_number, FileObject};
use anyhow::Result;
use bytes::Bytes;
use std::collections::BTreeMap;

mod logger;

pub struct MemTable {
    /// table is the sorted searching data structure
    /// `key` is the user record key, `value` is the user record value
    table: BTreeMap<Bytes, Bytes>,
    /// memtable unique sequence number, which represents the related log
    /// file number, it is global unique
    seq: u64,
    logger: MemTableLogger,
}

impl MemTable {
    pub fn new() -> Self {
        let seq = get_global_sequence_number();
        MemTable {
            table: BTreeMap::new(),
            seq,
            logger: MemTableLogger::new(seq),
        }
    }
    pub fn put(&mut self, key: &[u8], value: &[u8]) -> Result<()> {
        // logging and flushing to disk first
        let mut log_record_builder = LogRecordsBuilder::new();
        log_record_builder.add(OperationType::PUT, key, value)?;
        let ret = self
            .logger
            .log_and_sync(log_record_builder.build());

        assert!(ret.is_ok());
        self.table
            .insert(Bytes::copy_from_slice(key), Bytes::copy_from_slice(value));
        Ok(())
    }
    pub fn get(&self, key: &[u8]) -> Option<Bytes> {
        match self.table.get(key) {
            None => None,
            Some(value) => Some(value.clone()),
        }
    }

    /// delete is composed by putting a new record with zero length value portion
    pub fn delete(&mut self, key: &[u8]) -> Result<()> {
        self.put(key, "".as_bytes())
    }
    pub fn recover(file: &FileObject) -> Self {
        todo!()
    }

    pub fn seq_num(&self) -> u64 {
        self.seq
    }
}

#[cfg(test)]
mod tests;
