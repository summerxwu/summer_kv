use crate::blocks::{Blocks, SIZE_U16};
use crate::iterator::Iterator;
use bytes::Buf;
use std::cmp::Ordering;

/// RecordIterator yields the records in related blocks if the
/// iterator it self is valid after invoking next()

pub struct BlockRecordIterator<'a> {
    block: &'a Blocks,
    is_valid: bool,
    current_index: usize,
}
impl<'a> BlockRecordIterator<'a> {
    
    fn key_at_index(&self, index: usize) -> Result<&'a [u8], String> {
        if index >= self.block.num_of_elements {
            return Err(format!("given index out of range of block`:"));
        }

        let data_offset = self.block.offsets[index] as usize;
        let key_length = self.block.data[data_offset..data_offset + SIZE_U16]
            .as_ref()
            .get_u16() as usize;
        Ok(self.block.data[data_offset + SIZE_U16..data_offset + SIZE_U16 + key_length].as_ref())
    }
}

impl<'a> Iterator for BlockRecordIterator<'a> {
    type Item = &'a Blocks;

    fn new(arg: Self::Item) -> Self {
        BlockRecordIterator {
            block: arg,
            is_valid: false,
            current_index: 0,
        }
    }

    fn seek_to_first(&mut self) {
        if self.block.offsets.len() == 0 {
            self.is_valid = false;
        }
        self.current_index = 0;
        self.is_valid = true;
    }
    fn seek_to_last(&mut self) {
        if self.block.offsets.len() == 0 {
            self.is_valid = false;
        }
        self.current_index = self.block.offsets.len() - 1;
        self.is_valid = true;
    }
    fn seek_to_key(&mut self, key: &[u8]) -> bool{
        // TODO(summerxwu): Maybe use binary search is better
        if self.block.offsets.len() == 0 {
            self.is_valid = false;
            return false;
        }
        let mut iter_index = self.current_index;
        if self.key().cmp(key) == Ordering::Equal {
            return true;
        }
        iter_index = iter_index + 1;
        while (iter_index % self.block.offsets.len()) != self.current_index {
            if let Ok(ikey) = self.key_at_index(iter_index){
                if ikey.cmp(key) == Ordering::Equal {
                    self.current_index = iter_index;
                    self.is_valid = true;
                    return true
                }
            }
            iter_index = iter_index+1;
        }
        return false
    }
    fn is_valid(&self) -> bool {
        self.is_valid
    }
    fn next(&mut self) {
        if self.current_index + 1 >= self.block.num_of_elements {
            self.is_valid = false;
        }
        self.current_index = self.current_index + 1;
        self.is_valid = true;
    }

    fn prev(&mut self) {
        if self.current_index == 0 {
            self.is_valid = false;
        }
        self.current_index = self.current_index - 1;
        self.is_valid = true;
    }

    fn key(&self) -> &[u8] {
        let data_offset = self.block.offsets[self.current_index] as usize;
        let key_length = self.block.data[data_offset..data_offset + SIZE_U16]
            .as_ref()
            .get_u16() as usize;
        self.block.data[data_offset + SIZE_U16..data_offset + SIZE_U16 + key_length].as_ref()
    }

    fn value(&self) -> &[u8] {
        let data_offset = self.block.offsets[self.current_index] as usize;
        let key_length = self.block.data[data_offset..data_offset + SIZE_U16]
            .as_ref()
            .get_u16() as usize;
        let data_length = self.block.data
            [data_offset + SIZE_U16 + key_length..data_offset + SIZE_U16 + key_length + SIZE_U16]
            .as_ref()
            .get_u16() as usize;
        self.block.data[data_offset + SIZE_U16 + key_length + SIZE_U16
            ..data_offset + SIZE_U16 + key_length + SIZE_U16 + data_length]
            .as_ref()
    }
}
