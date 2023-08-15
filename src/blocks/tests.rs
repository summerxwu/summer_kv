use bytes::Bytes;
use crate::blocks::block_builder::BlockBuilder;
/// print the readable
fn extract_readable_info_from_buffer(bytes_buf :&Bytes){


}
#[test]
pub fn test_build_block() {
    let mut builder = BlockBuilder::new();
    builder.add(b"49",b"50").expect("Testing expect");
    let buf = builder.build().encode();
    println!("{:?}",buf.to_vec())
}
