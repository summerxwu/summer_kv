use crate::blocks::iterator::BlockRecordIterator;
use crate::blocks::Blocks;
use crate::iterator::Iterator;
use crate::sstable::SSTable;
use std::sync::Arc;

pub struct SSTableRecordIterator {
    sstable: Arc<SSTable>,
    data_block_iterator: BlockRecordIterator,
    data_block_index: usize,
}

impl Iterator for SSTableRecordIterator {
    type Item = Arc<SSTable>;
    fn new(sstable: Self::Item) -> Self {
        let data_block_index = 0;
        let offset = sstable.indexes[data_block_index].data_block_pointer.0 as u64;
        let length = sstable.indexes[data_block_index].data_block_pointer.1;
        let data_block_raw = sstable
            .file_object
            .read(offset, length)
            .expect("A SSTable file should contain one data block at least");

        let data_block = Arc::new(Blocks::decode(data_block_raw.as_ref()));
        let data_block_iterator = BlockRecordIterator::new(data_block);
        SSTableRecordIterator{
            sstable,
            data_block_iterator,
            data_block_index,
        }
    }
    fn seek_to_first(&mut self) {
        if self.data_block_index == 0{
            self.data_block_iterator.seek_to_first();
            return;
        }
        self.data_block_index = 0;
        let data_block_index = 0;
        let offset = self.sstable.indexes[data_block_index].data_block_pointer.0 as u64;
        let length = self.sstable.indexes[data_block_index].data_block_pointer.1;
        let data_block_raw = self.sstable
            .file_object
            .read(offset, length)
            .expect("A SSTable file should contain one data block at least");

        let data_block = Arc::new(Blocks::decode(data_block_raw.as_ref()));
        self.data_block_iterator = BlockRecordIterator::new(data_block);
    }

    fn seek_to_last(&mut self) {
    }

    fn seek_to_key(&mut self, key: &[u8]) -> bool {
        todo!()
    }

    fn is_valid(&self) -> bool {
        self.data_block_iterator.is_valid()
    }

    fn next(&mut self) {
        todo!()
    }

    fn prev(&mut self) {
        todo!()
    }

    fn key(&self) -> &[u8] {
        self.data_block_iterator.key()
    }

    fn value(&self) -> &[u8] {
        self.data_block_iterator.value()
    }
}
