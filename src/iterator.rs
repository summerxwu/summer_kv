pub trait Iterator {
    type Item;
    fn new(arg: Self::Item) -> Self;
    fn seek_to_first(&mut self);
    fn seek_to_last(&mut self);
    fn seek_to_key(&mut self, key: &[u8]) ;
    fn is_valid(&self) -> bool;
    fn next(&mut self);
    fn prev(&mut self);
    fn key(&self) -> &[u8];
    fn value(&self) -> &[u8];
}
