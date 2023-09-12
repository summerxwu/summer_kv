use crate::blocks::iterator::BlockRecordIterator;
use crate::blocks::Blocks;
use crate::iterator::Iterator;
use crate::sstable::SSTable;
use std::cmp::Ordering;
use std::sync::Arc;

pub struct SSTableRecordIterator {
    sstable: Arc<SSTable>,
    data_block_iterator: BlockRecordIterator,
    data_block_index: usize,
    is_valid: bool,
}

impl Iterator for SSTableRecordIterator {
    type Item = Arc<SSTable>;
    fn new(sstable: Self::Item) -> Self {
        let data_block_index = 0;
        let offset = sstable.indexes[data_block_index].data_block_pointer.0 as u64;
        let length = sstable.indexes[data_block_index].data_block_pointer.1;
        let data_block_raw = sstable
            .file_object
            .read_at(offset, length)
            .expect("A SSTable file should contain one data block at least");

        let data_block = Arc::new(Blocks::decode(data_block_raw.as_ref()));
        let data_block_iterator = BlockRecordIterator::new(data_block);
        SSTableRecordIterator {
            sstable,
            data_block_iterator,
            data_block_index,
            is_valid: false,
        }
    }
    fn seek_to_first(&mut self) {
        if self.data_block_index == 0 {
            self.data_block_iterator.seek_to_first();
            self.is_valid = true;
            return;
        }
        self.data_block_index = 0;
        let data_block_index = 0;
        let offset = self.sstable.indexes[data_block_index].data_block_pointer.0 as u64;
        let length = self.sstable.indexes[data_block_index].data_block_pointer.1;
        let data_block_raw = self
            .sstable
            .file_object
            .read_at(offset, length)
            .expect("A SSTable file should contain one data block at least");

        let data_block = Arc::new(Blocks::decode(data_block_raw.as_ref()));
        self.data_block_iterator = BlockRecordIterator::new(data_block);
        self.is_valid = true;
    }

    fn seek_to_last(&mut self) {
        if self.data_block_index == self.sstable.indexes.len() - 1 {
            self.data_block_iterator.seek_to_last();
            self.is_valid = true;
            return;
        }
        self.data_block_index = self.sstable.indexes.len() - 1;
        let data_block_index = self.data_block_index;
        let offset = self.sstable.indexes[data_block_index].data_block_pointer.0 as u64;
        let length = self.sstable.indexes[data_block_index].data_block_pointer.1;
        let data_block_raw = self
            .sstable
            .file_object
            .read_at(offset, length)
            .expect("A SSTable file should contain one data block at least");
        let data_block = Arc::new(Blocks::decode(data_block_raw.as_ref()));
        self.data_block_iterator = BlockRecordIterator::new(data_block);
        self.data_block_iterator.seek_to_last();
        self.is_valid = true;
    }

    fn seek_to_key(&mut self, key: &[u8]) {
        for index in &self.sstable.indexes {
            if key.cmp(index.largest_key.as_slice()) != Ordering::Greater {
                let offset = index.data_block_pointer.0 as u64;
                let length = index.data_block_pointer.1;
                let data_block_raw = self
                    .sstable
                    .file_object
                    .read_at(offset, length)
                    .expect("A SSTable file block pointer invalid");
                let data_block = Arc::new(Blocks::decode(data_block_raw.as_ref()));
                self.data_block_iterator = BlockRecordIterator::new(data_block);
                self.data_block_iterator.seek_to_key(key);
                if !self.data_block_iterator.is_valid() {
                    self.is_valid = false;
                    return;
                }
                self.is_valid = true;
                return;
            }
        }
        self.is_valid = false;
    }

    fn is_valid(&self) -> bool {
        self.is_valid
    }

    fn next(&mut self) {
        self.data_block_iterator.next();
        if self.data_block_iterator.is_valid() {
            self.is_valid = true;
            return;
        }
        if self.data_block_index + 1 < self.sstable.indexes.len() {
            self.data_block_index += 1;
            let data_block_index = self.data_block_index;
            let offset = self.sstable.indexes[data_block_index].data_block_pointer.0 as u64;
            let length = self.sstable.indexes[data_block_index].data_block_pointer.1;
            let data_block_raw = self
                .sstable
                .file_object
                .read_at(offset, length)
                .expect("A SSTable file should contain one data block at least");
            let data_block = Arc::new(Blocks::decode(data_block_raw.as_ref()));
            self.data_block_iterator = BlockRecordIterator::new(data_block);
            self.data_block_iterator.seek_to_first();
            self.is_valid = true;
        } else {
            self.is_valid = false;
        }
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
