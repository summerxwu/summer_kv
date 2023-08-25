use std::sync::Arc;
use crate::blocks::iterator::BlockRecordIterator;
use crate::blocks::{Blocks, SIZE_U16};
use crate::iterator::Iterator;
use crate::util::env;
use anyhow::Result;
use bytes::{Buf, BufMut, Bytes, BytesMut};

mod iterator;
mod sstable_builder;
pub type KVPair = (Bytes, Bytes);
/// # SSTable format
/// SSTable is a data structure that represent the disk file which hold the data in order.
/// SSTable disk layout is described below:
/// ``` text
/// <file begin>
/// <data block>
/// <data block>
/// ...
/// <data block>
/// <index block>
/// ...
/// <index block>
/// <footer>
/// <file end>
/// ```
/// ## index block
/// index block is basically a data block but the record is consist of a key and a `block pointer`
///
/// index block is of a record sorted.  The key of each record in index block is represent the
/// largest key of a data block in current SSTable, the value is the related data `block pointer`.
///
/// Records of the index block
/// ``` text
/// +----------------------------------------------------------------------------+
/// | Key Length#2| Key PayLoads(key-length bytes)| Value Length#2| Block Pointer|
/// +----------------------------------------------------------------------------+
/// ```
///
/// `block pointer` is consist of offset and size which are all `usize` type.
/// ```text
/// +-----------------+
/// | offset#2+size#2 |
/// +-----------------+
/// ```
///
/// ## footer
/// ``` text
/// +----------------------|----------------------|------|---------------------+
/// |index block pointer#4 |index block pointer#4 |------|index block number#2 |
/// +----------------------|----------------------|------|---------------------+
/// ```
/// So the decode of SSTable disk file procedure is like this:
/// - read the last 2 bytes to get the index block size.
/// - based on the index block size, fetch all block pointer point to the index block.
/// - decode every index block which their records represents data block with mata data of
/// largest key, offset and size of the block.
/// - Searching a user records can start with binary searching with the data block meta data
/// to determine which data block contain the demanded user records and iterate the records of
/// the data block to fetch the result
///

pub struct SSTable {
    file_object: env::FileObject,
    indexes: Vec<IndexBlockRecord>,
    seq: usize,
}
impl SSTable {
    /// create a new SSTable object by a exists disk file identified by sequence number
    fn open(seq: usize) -> Result<Self> {
        let file_path = env::sstfile_path(seq);
        let file_object = env::FileObject::open(file_path.as_str())?;

        // Initialize the `indexes` field
        // init the footer
        let mut buf = file_object.read_last_of(2)?;
        let index_block_num = buf.get_u16() as usize;

        let footer_length = index_block_num * std::mem::size_of::<BlockPointer>() + 2;
        let footer_buf = file_object.read_last_of(footer_length)?;

        let mut indexes: Vec<IndexBlockRecord> = Vec::new();

        let footer_obj = Footer::decode(footer_buf.as_ref());
        // read records of each index block pointed by footer
        for index_block_pointer in &footer_obj.index_block_pointers {
            //read the index block
            let buf = file_object.read(index_block_pointer.0 as u64, index_block_pointer.1 as usize)?;
            let index_block_obj = Arc::new(Blocks::decode(buf.as_ref().clone()));
            let mut record_iter = BlockRecordIterator::new(index_block_obj);
            while record_iter.is_valid() {
                let record = IndexBlockRecord {
                    largest_key: record_iter.key().to_vec().clone(),
                    data_block_pointer: BlockPointer::decode(record_iter.value()),
                };
                indexes.push(record);
                record_iter.next();
            }
        }
        Ok(SSTable {
            file_object,
            indexes,
            seq,
        })
    }
    fn get(&self, key: &[u8]) -> Result<&[u8]> {
        todo!()
    }
}

struct IndexBlockRecord {
    largest_key: Vec<u8>,
    data_block_pointer: BlockPointer,
}

impl IndexBlockRecord {
    fn encode(&self) -> Bytes {
        let mut buf = BytesMut::new();
        buf.put_u16(self.largest_key.len() as u16);
        buf.put_slice(self.largest_key.as_slice());

        let block_pointer_seq = self.data_block_pointer.encode();
        buf.put_u16(block_pointer_seq.len() as u16);
        buf.put_slice(block_pointer_seq.as_ref());
        buf.freeze()
    }
    fn decode(raw: &[u8]) -> Self {
        let mut buf = &raw[..];
        let key_length = buf.get_u16() as usize;
        let key_buf = &raw[SIZE_U16..SIZE_U16 + key_length];
        let mut key_vec: Vec<u8> = Vec::new();
        for i in key_buf {
            key_vec.push(i.clone());
        }

        let value_buf = &raw[SIZE_U16 + key_length + SIZE_U16..];
        let block_pointer = BlockPointer::decode(value_buf);
        IndexBlockRecord {
            largest_key: key_vec,
            data_block_pointer: block_pointer,
        }
    }
}
struct Footer {
    index_block_pointers: Vec<BlockPointer>,
    num_of_index_block: usize,
}

impl Footer {
    fn encode(&self) -> Bytes {
        let mut buf = BytesMut::new();

        for index_block_pointer in &self.index_block_pointers {
            buf.put_slice(index_block_pointer.encode().as_ref());
        }

        buf.put_u16(self.num_of_index_block as u16);
        buf.freeze()
    }
    fn decode(raw: &[u8]) -> Footer {
        use std::iter::Iterator;
        let mut last_two_bytes = &raw[raw.len() - SIZE_U16..];
        let num_of_index_block = last_two_bytes.get_u16() as usize;

        let raw_index_block_pointers = &raw[..raw.len() - SIZE_U16];
        let buf_vec = raw_index_block_pointers
            .chunks(std::mem::size_of::<BlockPointer>())
            .map(|x| BlockPointer::decode(x))
            .collect();
        Footer {
            index_block_pointers: buf_vec,
            num_of_index_block,
        }
    }
}
/// offset and length
#[derive(Debug)]
struct BlockPointer(usize, usize);
impl BlockPointer {
    fn encode(&self) -> Bytes {
        let mut buf = BytesMut::new();
        buf.put_slice(self.0.to_be_bytes().as_ref());
        buf.put_slice(self.1.to_be_bytes().as_ref());
        buf.freeze()
    }
    fn decode(raw: &[u8]) -> Self {
        let mut buf = &raw[..];
        let offset = buf.get_uint(std::mem::size_of::<usize>()) as usize;
        let size = buf.get_uint(std::mem::size_of::<usize>()) as usize;
        BlockPointer(offset, size)
    }
}
impl PartialEq for BlockPointer{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
#[cfg(test)]
mod tests;
