use crate::util::env::{logfile_path, FileObject};
use anyhow::Result;
use bytes::{BufMut, Bytes, BytesMut};
use std::thread::sleep;

pub struct MemTableLogger {
    seq: u64,
    file_obj: FileObject,
}
impl MemTableLogger {
    pub fn new(seq: u64) -> Self {
        let file_obj = FileObject::create(logfile_path(seq as usize).as_str()).unwrap();
        MemTableLogger { seq, file_obj }
    }
    pub fn log_and_sync(& mut self, log_records: &[u8]) -> Result<()> {
        self.file_obj.write(log_records)?;
        self.file_obj.sync()
    }
}

pub struct LoggerRecord {
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

/// LogRecords is a sequence of `Entity`
/// `Entity` format is described below
///
/// ``` text
/// +----------------------------------------------------------------------------------------------------------------------+
/// | Key Length (2 bytes) |  Key PayLoads(key-length bytes) | Value Length (2 bytes) | Value PayLoad (value-length bytes) |
/// +----------------------------------------------------------------------------------------------------------------------+
/// ```
/// It is same as the block entry, if the `Value Length` portion is zero, yields a delete operation
/// on that key
pub struct LogRecordsBuilder {
    data: Vec<u8>,
}
impl LogRecordsBuilder {
    pub fn new() -> Self {
        LogRecordsBuilder { data: Vec::new() }
    }
    pub fn add(&mut self, opt: OperationType, key: &[u8], value: &[u8]) -> Result<()> {
        let mut buf = BytesMut::new();
        //encoding key portion of the records
        buf.put_u16(key.len() as u16);
        buf.put_slice(key);
        // encoding value portion of the records
        match opt {
            OperationType::PUT => {
                buf.put_u16(value.len() as u16);
                buf.put_slice(value);
            }
            OperationType::DELETE => {
                buf.put_u16(0);
            }
        }
        self.data.put_slice(buf.as_ref());
        Ok(())
    }
    pub fn build(&self) -> &[u8] {
        self.data.as_slice()
    }
    pub fn cleanup(&mut self) {
        self.data.clear();
    }
}
