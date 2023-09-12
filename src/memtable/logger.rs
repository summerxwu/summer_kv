use crate::blocks::SIZE_U16;
use crate::memtable::logger::OperationType::{DELETE, PUT};
use crate::util::env::{logfile_path, FileObject};
use anyhow::Result;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::fmt::Formatter;

pub struct MemTableLogger {
    seq: u64,
    file_obj: FileObject,
}
impl MemTableLogger {
    pub fn new(seq: u64) -> Self {
        let file_obj = FileObject::create(logfile_path(seq as usize).as_str()).unwrap();
        MemTableLogger { seq, file_obj }
    }
    pub fn log_and_sync(&mut self, log_records: &[u8]) -> Result<()> {
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
    pub fn new(opt: OperationType, key: &[u8], value: &[u8]) -> Self {
        LoggerRecord {
            opt_type: opt,
            key: Bytes::copy_from_slice(key),
            value: Bytes::copy_from_slice(value),
        }
    }
    pub fn encode(&self) -> Bytes {
        let mut buf = BytesMut::new();
        //encoding key portion of the records
        buf.put_u16(self.key.len() as u16);
        buf.put_slice(self.key.as_ref());
        // encoding value portion of the records
        match self.opt_type {
            PUT => {
                buf.put_u16(self.value.len() as u16);
                buf.put_slice(self.value.as_ref());
            }
            DELETE => {
                buf.put_u16(0);
            }
        }
        buf.freeze()
    }

    pub fn decode(buf: &[u8]) -> Self {
        let raw = buf;
        let key_length = raw[..SIZE_U16].as_ref().get_u16();
        let key_raw = raw[SIZE_U16..SIZE_U16 + key_length as usize].as_ref();
        let key = Bytes::copy_from_slice(key_raw);
        let value_portion = raw[SIZE_U16 + key_length as usize..].as_ref();
        let value_length = value_portion[..SIZE_U16].as_ref().get_u16();
        if value_length == 0 {
            return LoggerRecord {
                opt_type: DELETE,
                key,
                value: Bytes::copy_from_slice("".as_bytes()),
            };
        }
        return LoggerRecord {
            opt_type: PUT,
            key,
            value: Bytes::copy_from_slice(value_portion[SIZE_U16..SIZE_U16+value_length as usize].as_ref()),
        };
    }
}

impl std::fmt::Display for LoggerRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "===\nopt: {:?}\nkey: {:?}\nvalue: {:?}\n===",
            self.opt_type, self.key, self.value,
        )
    }
}

#[derive(Default, Debug)]
pub enum OperationType {
    #[default]
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
        let record = LoggerRecord::new(opt, key, value);

        self.data.put_slice(record.encode().as_ref());
        Ok(())
    }
    pub fn build(&self) -> &[u8] {
        self.data.as_slice()
    }
    pub fn cleanup(&mut self) {
        self.data.clear();
    }
}
