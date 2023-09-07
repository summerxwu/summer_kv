use anyhow::Result;
use bytes::Bytes;

pub trait DB {
    fn open(&self,path: &str) ->Result<()>;
    fn close(&self);
    fn put(&mut self, key: &[u8], value: &[u8]) -> Result<()>;
    fn get(&self, key: &[u8]) -> Option<Bytes>;
    fn delete(&self, key: &[u8]) -> Result<()>;
}

pub struct DBImpl {}

impl DBImpl {
    pub fn new() ->Self{
        todo!()
    }

}
impl DB for DBImpl {
    fn open(&self,path: &str) ->Result<()> {
        todo!()
    }

    fn close(&self) {
        todo!()
    }

    fn put(&mut self, key: &[u8], value: &[u8]) -> Result<()> {
        todo!()
    }

    fn get(&self, key: &[u8]) -> Option<Bytes> {
        todo!()
    }

    fn delete(&self, key: &[u8]) -> Result<()> {
        todo!()
    }
}
