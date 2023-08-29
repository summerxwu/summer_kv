use crate::util::env::{logfile_path, FileObject};
use anyhow::Result;
use bytes::Bytes;

pub struct MemTableLogger {
    seq: u64,
    file_obj: FileObject,
}
impl MemTableLogger {
    pub fn new(seq: u64) -> Self {
        let file_obj = FileObject::create(logfile_path(seq as usize).as_str()).unwrap();
        MemTableLogger { seq, file_obj }
    }
    pub fn log_and_sync(&self, log_records: &[LoggerRecord]) -> Result<()> {
        todo!()
    }
}

struct LoggerRecord {
    opt_type: OperationType,
    key: Bytes,
    value: Bytes,
}
impl LoggerRecord {
    fn encode(&self) -> Bytes {
        todo!()
    }
    fn decode(buf: &[u8]) -> Self {
        todo!()
    }
}

pub enum OperationType {
    PUT,
    DELETE,
}

pub struct LogRecordsBuilder {}
impl LogRecordsBuilder {
    pub fn new() -> Self {
        todo!()
    }
    pub fn add(&mut self, opt: OperationType, key: &[u8], value: &[u8]) -> Result<()> {
        todo!()
    }
    pub fn build(&self) -> Vec<LoggerRecord> {
        todo!()
    }
}
