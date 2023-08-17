mod sstable_builder;
mod block_iterator;

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
///

pub struct SSTable{
    index:usize,
}

#[cfg(test)]
mod tests;