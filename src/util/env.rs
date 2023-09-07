use anyhow::Result;
use bytes::{Bytes, BytesMut};
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Write};
use std::os::unix::fs::FileExt;
use std::sync::atomic::{AtomicU64, Ordering};

static GLOBAL_SEQUENCE_NUMBER: AtomicU64 = AtomicU64::new(0);
pub struct FileObject {
    file_handler: File,
}
impl FileObject {
    /// Create a new FileObject by a given file name.
    /// Failed if file exists.
    pub fn create(path: &str) -> Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(path)?;
        Ok(FileObject { file_handler: file })
    }

    /// open a new FileObject by a given file name.
    /// Failed if file not exists.
    pub fn open(path: &str) -> Result<Self> {
        let file_handler = OpenOptions::new().read(true).write(true).open(path)?;
        let size = file_handler.metadata()?.len();
        Ok(FileObject { file_handler })
    }

    /// Read content from file from `offset` by `length` long
    pub fn read(&self, offset: u64, length: usize) -> Result<Bytes> {
        let mut buf = BytesMut::zeroed(length);
        self.file_handler.read_exact_at(buf.as_mut(), offset)?;
        Ok(buf.freeze())
    }

    /// Read Last length bytes of file content
    pub fn read_last_of(&self, length: usize) -> Result<Bytes> {
        let offset = self.file_handler.metadata()?.len() - length as u64;
        self.read(offset, length)
    }

    /// Write Content to file
    pub fn write(&mut self, buf: &[u8]) -> Result<()> {
        let ret = self.file_handler.seek(SeekFrom::End(0))?;
        self.file_handler.write(buf)?;
        Ok(())
    }
    // Do fsync(), flush data to disk
    pub fn sync(&self) -> Result<()> {
        self.file_handler.sync_all()?;
        Ok(())
    }

    // Return the approximate size of current file
    pub fn size(&self) -> u64 {
        self.file_handler.metadata().unwrap().len()
    }
}
pub fn sstfile_path(seq: usize) -> String {
    format!("/tmp/summer_kv_test/{}.sst", seq)
}
pub fn logfile_path(seq: usize) -> String {
    format!("/tmp/summer_kv_test/{}.log", seq)
}
pub fn get_global_sequence_number() -> u64 {
    GLOBAL_SEQUENCE_NUMBER.fetch_add(1, Ordering::SeqCst).into()
}

#[cfg(test)]
mod tests {
    use crate::util::env::{get_global_sequence_number, FileObject};
    use std::fs;
    use std::io::Write;

    const TMP_FILE: &str = "/tmp/test.test";
    /// create file with amount of content of size `size` M
    fn create_tmp_file(size: usize) {
        let mut f = FileObject::create(TMP_FILE).unwrap();
        for l in 0..size {
            let buf: [u8; 1024] = [(l + 1) as u8; 1024];
            f.file_handler.write(&buf).unwrap();
        }
        f.file_handler.sync_all().unwrap();
    }
    fn remove_tmp_file() {
        let _ = fs::remove_file(TMP_FILE);
    }
    struct RaiiFinalize {}
    impl Drop for RaiiFinalize {
        fn drop(&mut self) {
            remove_tmp_file()
        }
    }
    #[test]
    fn test_create() {
        let raii = RaiiFinalize {};
        let ret = FileObject::create(TMP_FILE);
        assert!(ret.is_ok());
        let ret = FileObject::create(TMP_FILE);
        assert!(ret.is_err());
    }
    #[test]
    fn test_open() {
        let raii = RaiiFinalize {};
        create_tmp_file(4);
        let ret = FileObject::open(TMP_FILE);
        assert!(ret.is_ok());
    }
    #[test]
    fn test_read() {
        let raii = RaiiFinalize {};
        create_tmp_file(4);
        let ret = FileObject::open(TMP_FILE);
        assert!(ret.is_ok());
        let file_obj = ret.unwrap();
        let ret = file_obj.read(1023, 4);
        assert!(ret.is_ok());
        let buf = ret.unwrap();
        assert_eq!(buf.to_vec().len(), 4);
        let result = format!("{:?}", buf.to_vec());
        assert_eq!(result, "[1, 2, 2, 2]");
    }

    #[test]
    fn test_write_and_read_last() {
        let raii = RaiiFinalize {};
        create_tmp_file(1);
        let ret = FileObject::open(TMP_FILE);
        assert!(ret.is_ok());
        let mut file_obj = ret.unwrap();
        file_obj
            .write("abcdefg".as_bytes())
            .expect("Testing expect");
        let ret = file_obj.read_last_of(7).expect("Testing expect");
        assert_eq!(ret.len(), 7);
        assert_eq!(ret.as_ref(), "abcdefg".as_bytes())
    }

    #[test]
    fn test_size() {
        let raii = RaiiFinalize {};
        create_tmp_file(1);
        let ret = FileObject::open(TMP_FILE);
        assert!(ret.is_ok());
        let mut file_obj = ret.unwrap();
        file_obj
            .write("abcdefg".as_bytes())
            .expect("Testing expect");
        assert_eq!(1031, file_obj.size())
    }

    #[test]
    fn test_global_seq_fetch() {
        assert_eq!(0, get_global_sequence_number());
        assert_eq!(1, get_global_sequence_number());
        assert_eq!(2, get_global_sequence_number());
    }
}
