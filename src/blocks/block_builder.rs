use crate::blocks::{Blocks, SIZE_U16};
use bytes::BufMut;

pub const BLOCK_SIZE: usize = 4 * 1024;

/// BlockBuilder accept the client provided records and generate the Block object
/// after invoking build().
/// Blocks layouts like this:
/// ``` text
/// ----------------------------------------------------------------------------------------------------
/// |             Data Section             |              Offset Section             |      Extra      |
/// ----------------------------------------------------------------------------------------------------
/// | Entry #1 | Entry #2 | ... | Entry #N | Offset #1 | Offset #2 | ... | Offset #N | num_of_elements |
/// ----------------------------------------------------------------------------------------------------
/// ```
///
/// Entry encoding format is described below:
/// ``` text
/// +----------------------------------------------------------------------------------------------------------------------+
/// | Key Length (2 bytes) |  Key PayLoads(key-length bytes) | Value Length (2 bytes) | Value PayLoad (value-length bytes) |
/// +----------------------------------------------------------------------------------------------------------------------+
/// ```
/// Every record will be encoded in this format and save the raw bytes into the data field.

pub struct BlockBuilder {
    data: Vec<u8>,
    offsets: Vec<u16>,
    amount: usize,
}
impl BlockBuilder {
    pub fn new() -> Self {
        BlockBuilder {
            data: Vec::new(),
            offsets: Vec::new(),
            // the Extra field occupy
            amount: 2,
        }
    }
    /// return the length of bytes sequence after encoding the origin one
    fn evaluate_record_encoded_length(key: &[u8], value: &[u8]) -> usize {
        SIZE_U16 + key.len() + SIZE_U16 + value.len()
    }

    /// add function will encode the `key` and `value` into the format described previously
    pub fn add(&mut self, key: &[u8], value: &[u8]) -> Result<(), &str> {
        // if the amount of bytes of current blocks exceeds the capacity of block limits
        // return error with info, If the first record inserting to current block exceeds
        // the block limits, current block will be extended
        if Self::evaluate_record_encoded_length(key, value) + self.amount  > BLOCK_SIZE {
            return Err("block overflow");
        }

        // write the offset of current record
        self.offsets.push((self.amount -2) as u16);

        // encoding key part
        let key_length = key.len() as u16;
        self.data.put_u16(key_length);
        for iter in key {
            self.data.push(iter.clone());
        }

        // encoding value part
        let value_length = value.len() as u16;
        self.data.put_u16(value_length);
        for iter in value {
            self.data.push(iter.clone());
        }

        // increasing the amount base on actual encoded length
        self.amount = self.amount + Self::evaluate_record_encoded_length(key,value);
        return Ok(());
    }
    pub fn build(&self) -> Blocks {
        let mut buf = self.data.clone();
        for offset in &self.offsets {
            buf.put_u16(*offset);
        }
        buf.put_u16(self.offsets.len()as u16);

        Blocks{
            data:self.data.clone(),
            offsets:self.offsets.clone(),
            num_of_elements: self.offsets.len(),
        }
    }
}
