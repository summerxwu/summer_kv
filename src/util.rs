pub mod error;
pub mod env;

pub trait Iterator {
    fn seek_to_first(&mut self);
    fn seek_to_last(&mut self);
    fn seek_to_key(&mut self, key: &[u8])->bool;
    fn is_valid(&self) -> bool;
    fn next(&mut self);
    fn prev(&mut self);
    fn key(&self) -> &[u8];
    fn value(&self) -> &[u8];
}

