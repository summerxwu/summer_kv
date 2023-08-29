use crate::memtable::MemTable;
use crate::util::env::logfile_path;

struct RAII {
    pub seq: u64,
}

impl Drop for RAII {
    fn drop(&mut self) {
        std::fs::remove_file(logfile_path(self.seq as usize)).expect("Testing expect");
    }
}
#[test]
fn test_memtable_put() {
    let mut memtable = MemTable::new();
    let raii = RAII {
        seq: memtable.seq_num(),
    };
    let ret = memtable.put("key1".as_bytes(), "value1".as_bytes());
    assert!(ret.is_ok())
}
#[test]
fn test_memtable_get() {
    let mut memtable = MemTable::new();
    let raii = RAII {
        seq: memtable.seq_num(),
    };
    let ret = memtable.put("key1".as_bytes(), "value1".as_bytes());
    assert!(ret.is_ok());
    assert_eq!(memtable.get("key1".as_bytes()),Some("value1".into()));
    assert_eq!(memtable.get("key".as_bytes()),None);
}
#[test]
fn test_memtable_remove() {
    let mut memtable = MemTable::new();
    let raii = RAII {
        seq: memtable.seq_num(),
    };
    let ret = memtable.put("key1".as_bytes(), "value1".as_bytes());
    assert!(ret.is_ok());
    assert_eq!(memtable.get("key1".as_bytes()),Some("value1".into()));
    assert_eq!(memtable.get("key".as_bytes()),None);
    let ret = memtable.delete("key1".as_bytes());
    assert!(ret.is_ok());
    assert_eq!(memtable.get("key1".as_bytes()),Some("".into()));

}
