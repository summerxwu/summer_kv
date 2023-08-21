use crate::sstable::iterator::SSTableRecordIterator;
use crate::sstable::sstable_builder::SSTableBuilder;
use crate::sstable::SSTable;
use crate::util::env::sstfile_path;
use std::fs;
use crate::iterator::Iterator;

struct TestSSTable {
    sstable: SSTable,
    builder: SSTableBuilder,
    record_num: u16,
}
impl TestSSTable {
    pub fn create_for_test(seq: usize, number: u16) -> Self {
        let mut builder = SSTableBuilder::new();
        for i in 0..number {
            builder.add(
                format!("key_{}", i + 1).as_bytes(),
                format!("value_{}", i + 1).as_bytes(),
            );
        }
        let sstable = builder.build().unwrap();
        TestSSTable {
            sstable,
            builder,
            record_num,
        }
    }
}
impl Drop for TestSSTable {
    fn drop(&mut self) {
        fs::remove_file(sstfile_path(self.sstable.seq)).expect("Testing expect");
    }
}
#[test]
fn test_build_sstable_one_record() {
    TestSSTable::create_for_test(1, 1);
}
#[test]
fn test_build_sstable_multi_records() {
    TestSSTable::create_for_test(1, 100);
}

#[test]
fn test_open_exists_sstable() {
    let test_sstable = TestSSTable::create_for_test(1, 10);
    SSTable::open(test_sstable.sstable.seq).unwrap();
}
#[test]
fn test_open_non_exists_sstable() {
    let ret = SSTable::open(2);
    assert!(ret.is_err());
}
#[test]
fn test_sstable_iterator() {
    let test_sstable = TestSSTable::create_for_test(1,100);
    let sstable = SSTable::open(test_sstable.sstable.seq).unwrap();
    let mut sstable_iter =SSTableRecordIterator::new(& sstable);
    sstable_iter.seek_to_first();
    assert_eq!(sstable_iter.key(),b"key_1".as_slice());
    sstable_iter.next();
    assert!(sstable_iter.is_valid());
    assert_eq!(sstable_iter.key(),b"key_2".as_slice());
    assert_eq!(sstable_iter.value(),b"value_2".as_slice());
}
#[test]
fn test_sstable_seek() {
    let test_sstable = TestSSTable::create_for_test(1,100);
    let sstable = SSTable::open(test_sstable.sstable.seq).unwrap();
    let mut sstable_iter =SSTableRecordIterator::new(& sstable);
    sstable_iter.seek_to_first();
    assert_eq!(sstable_iter.key(),b"key_1".as_slice());
    sstable_iter.seek_to_key(b"key_53".as_slice());
    assert!(sstable_iter.is_valid());
    assert_eq!(sstable_iter.key(),b"key_53".as_slice());
    assert_eq!(sstable_iter.value(),b"value_53".as_slice());
}
#[test]
fn test_sstable_get() {
    let test_sstable = TestSSTable::create_for_test(1, 10);
    let sstable = SSTable::open(test_sstable.sstable.seq).unwrap();
    let value = sstable.get(b"key_5").unwrap();
    assert_eq!(value,b"value_5".as_slice());
    let value = sstable.get(b"key_not_found");
    assert!(value.is_err());
}
