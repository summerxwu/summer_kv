use crate::sstable::SSTable;
use crate::iterator::Iterator;
pub struct SSTableRecordIterator<'a>{
    sstable : &'a SSTable,
    is_valid: bool,
}

impl<'a> Iterator for SSTableRecordIterator<'a> {
    type Item = &'a SSTable;
    fn new( sstable : Self::Item) -> Self{
        SSTableRecordIterator {
            sstable,
            is_valid: false
        }
    }
    fn seek_to_first(&mut self) {
        todo!()
    }

    fn seek_to_last(&mut self) {
        todo!()
    }

    fn seek_to_key(&mut self, key: &[u8]) -> bool {
        todo!()
    }

    fn is_valid(&self) -> bool {
        todo!()
    }

    fn next(&mut self) {
        todo!()
    }

    fn prev(&mut self) {
        todo!()
    }

    fn key(&self) -> &[u8] {
        todo!()
    }

    fn value(&self) -> &[u8] {
        todo!()
    }
}