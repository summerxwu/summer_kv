use crate::blocks::block_builder::BlockBuilder;
use crate::blocks::iterator::BlockRecordIterator;
use crate::blocks::Blocks;
use crate::iterator::Iterator;
use std::sync::Arc;

fn create_block_with_rec_num(size: u8) -> Arc<Blocks> {
    let mut builder = BlockBuilder::new();
    for i in 0..size {
        builder
            .add(
                format!("key_{}", i + 1).as_ref(),
                format!("value_{}", i + 1).as_ref(),
            )
            .expect("Testing expect");
    }
    Arc::new(builder.build())
}

#[test]
fn test_build_block() {
    let mut builder = BlockBuilder::new();
    builder.add(b"49", b"50").expect("Testing expect");
    let buf = builder.build().encode();
    println!("{:?}", buf.to_vec())
}

#[test]
fn test_iterator_create() {
    let block = create_block_with_rec_num(1);
    let iter = BlockRecordIterator::new(block);
}

#[test]
fn test_iterator_seek_to_first() {
    let block = create_block_with_rec_num(10);
    let mut iter = BlockRecordIterator::new(block);
    iter.seek_to_first();
    assert_eq!("key_1".as_bytes(), iter.key());
}
#[test]
fn test_iterator_seek_to_last() {
    let block = create_block_with_rec_num(10);
    let mut iter = BlockRecordIterator::new(block);
    iter.seek_to_last();
    assert_eq!("key_10".as_bytes(), iter.key());
}

#[test]
fn test_iterator_seek_to_key() {
    let block = create_block_with_rec_num(10);
    let mut iter = BlockRecordIterator::new(block);
    iter.seek_to_key("key_7".as_ref());
    assert_eq!("key_7".as_bytes(), iter.key());
}
#[test]
fn test_iterator_next() {
    let block = create_block_with_rec_num(10);
    let mut iter = BlockRecordIterator::new(block);
    iter.seek_to_key("key_5".as_ref());
    assert!(iter.is_valid());
    assert_eq!("key_5".as_bytes(), iter.key());
    iter.next();
    assert!(iter.is_valid());
    assert_eq!("key_6".as_bytes(), iter.key());
    println!("finish")
}

#[test]
fn test_block_boundary() {
    let block = create_block_with_rec_num(10);
    assert_eq!("key_1".as_bytes(),block.smallest_key());
    assert_eq!("key_10".as_bytes(),block.largest_key());
}
