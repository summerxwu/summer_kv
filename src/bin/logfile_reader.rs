use anyhow::Result;
use bytes::Buf;
use std::io::Write;
use summer_kv::blocks::SIZE_U16;
use summer_kv::memtable::logger::LoggerRecord;
use summer_kv::util::env::FileObject;

struct LogReader {
    file_obj: FileObject,
}

impl LogReader {
    pub fn new(path: &str) -> Self {
        let file_obj = FileObject::open(path).expect("Log_reader open log file");
        LogReader { file_obj }
    }
    pub fn print<T>(&self, dest: &mut T, log_record: &LoggerRecord)
    where
        T: Write,
    {
        dest.write(log_record.to_string().as_bytes()).expect("");
        dest.flush().expect("");
    }

    pub fn next_record(&mut self) -> Result<LoggerRecord> {
        let mut key_length_buf = self.file_obj.read(SIZE_U16)?;
        let key_length = key_length_buf.get_u16();
        let key_buf = self.file_obj.read(key_length as usize)?;
        let mut value_length_buf = self.file_obj.read(SIZE_U16)?;
        let value_length = value_length_buf.get_u16();
        let value_buf = self.file_obj.read(value_length as usize)?;

        let binding = [
            key_length.to_be_bytes().as_ref(),
            key_buf.as_ref(),
            value_length.to_be_bytes().as_ref(),
            value_buf.as_ref(),
        ]
        .concat();
        let compact_buf = binding.as_slice();
        let record = LoggerRecord::decode(compact_buf);
        Ok(record)
    }
}

fn main() {
    let mut obj = LogReader::new("/tmp/summer_kv_test/0.log");
    loop {
        if let Ok(record) = obj.next_record() {
            obj.print(&mut std::io::stdout(), &record);
        } else {
            break;
        }
    }
}
