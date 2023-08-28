use crate::util::env::FileObject;
use anyhow::Result;
use bytes::Bytes;

pub struct MemLogger {
    seq: u64,
    file_obj: FileObject,
}
impl MemLogger {
    fn new(seq: u64) -> Self {
        todo!()
    }
    fn log_and_sync(&self, log_records: &[LoggerRecord]) -> Result<()> {
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

enum OperationType {
    PUT,
    DELETE,
}

pub struct LogRecordsBuilder {}
impl LogRecordsBuilder {
    fn new() -> Self {
        todo!()
    }
    fn add(&mut self, key: &[u8], value: &[u8]) -> Result<()> {
        todo!()
    }
    fn build() -> Vec<LoggerRecord> {
        todo!()
    }
}
