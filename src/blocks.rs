mod block_builder;
mod iterator;

use bytes::{Buf, BufMut, Bytes};

/// A block is the smallest unit of read and caching in LSM tree.
/// It is a collection of sorted key-value pairs.
/// The `actual` storage format is as below (After `Block::encode`):
///
/// ``` text
/// ----------------------------------------------------------------------------------------------------
/// |             Data Section             |              Offset Section             |      Extra      |
/// ----------------------------------------------------------------------------------------------------
/// | Entry #1 | Entry #2 | ... | Entry #N | Offset #1 | Offset #2 | ... | Offset #N | num_of_elements |
/// ----------------------------------------------------------------------------------------------------
/// ```
/// The `Entry` storage format is as below (After `BlockBuilder::add()`):
///
/// ``` text
/// +----------------------------------------------------------------------------------------------------------------------+
/// | Key Length (2 bytes) |  Key PayLoads(key-length bytes) | Value Length (2 bytes) | Value PayLoad (value-length bytes) |
/// +----------------------------------------------------------------------------------------------------------------------+
/// ```

pub const SIZE_U16: usize = std::mem::size_of::<u16>();
pub struct Blocks {
    data: Vec<u8>,
    offsets: Vec<u16>,
    num_of_elements: usize,
}

impl Blocks {
    pub fn encode(&self) -> Bytes {
        let mut buf = self.data.clone();
        for offset in &self.offsets {
            buf.put_u16(*offset);
        }
        let num_of_element = self.offsets.len() as u16;
        buf.put_u16(num_of_element);
        buf.into()
    }
    pub fn decode(data: &[u8]) -> Self {
        let mut footer = &data[data.len() - SIZE_U16..];
        let num_of_elements = footer.get_u16() as usize;

        let offsets_portion =
            &data[data.len() - SIZE_U16 - num_of_elements * SIZE_U16..data.len() - SIZE_U16];

        let data_portion = &data[..data.len() - SIZE_U16 - num_of_elements * SIZE_U16];
        Blocks {
            data: data_portion.into(),
            offsets: offsets_portion
                .chunks(SIZE_U16)
                .map(|mut x| x.get_u16())
                .collect(),
            num_of_elements,
        }
    }
}

#[cfg(test)]
mod tests;
