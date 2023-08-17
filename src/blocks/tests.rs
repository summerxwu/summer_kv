use crate::blocks::block_builder::BlockBuilder;
use crate::blocks::iterator::RecordIterator;
use crate::blocks::Blocks;
use crate::util::Iterator;

fn create_block_with_rec_num(size: u8) -> Blocks {
    let mut builder = BlockBuilder::new();
    for i in 0..size {
        builder
            .add(
                format!("key_{}", i+1).as_ref(),
                format!("value_{}", i+1).as_ref(),
            )
            .expect("Testing expect");
    }
    builder.build()
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
    let iter = RecordIterator::new(&block);
}

#[test]
fn test_iterator_seek_to_first() {
    let block = create_block_with_rec_num(10);
    let mut iter = RecordIterator::new(&block);
    iter.seek_to_first();
    assert_eq!("key_1".as_bytes(), iter.key());
}
#[test]
fn test_iterator_seek_to_last() {
    let block = create_block_with_rec_num(10);
    let mut iter = RecordIterator::new(&block);
    iter.seek_to_last();
    assert_eq!("key_10".as_bytes(), iter.key());
}

#[test]
fn test_iterator_seek_to_key(){
    let block = create_block_with_rec_num(10);
    let mut iter = RecordIterator::new(&block);
    iter.seek_to_key("key_7".as_ref());
    assert_eq!("key_7".as_bytes(),iter.key());
}
#[test]
fn test_iterator_next() {
    let block = create_block_with_rec_num(10);
    let mut iter = RecordIterator::new(&block);
    iter.seek_to_key("key_5".as_ref());
    assert!(iter.is_valid());
    assert_eq!("key_5".as_bytes(),iter.key());
    iter.next();
    assert!(iter.is_valid());
    assert_eq!("key_6".as_bytes(),iter.key());
}
