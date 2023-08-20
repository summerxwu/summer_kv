use crate::sstable::sstable_builder::SSTableBuilder;
use crate::sstable::SSTable;
use crate::util::env::sstfile_path;
use std::fmt::format;
use std::fs;
use crate::sstable::block_iterator::BlockIterator;

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
    TestSSTable::create_for_test(1,1);
}
#[test]
fn test_build_sstable_multi_records() {
    TestSSTable::create_for_test(1,100);
}

#[test]
fn test_open_exists_sstable() {
    let test_sstable = TestSSTable::create_for_test(1,10);
    SSTable::open(test_sstable.sstable.seq).unwrap();
}
#[test]
fn test_open_non_exists_sstable() {
    let ret = SSTable::open(2);
    assert!(ret.is_err());
}
#[test]
fn test_sstable_iterator() {
    let test = TestSSTable::create_for_test(1,100);
    let iter = BlockIterator::new(&(test.sstable));
}
#[test]
fn test_sstable_seek() {}
#[test]
fn test_sstable_get() {}
