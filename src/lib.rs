#![allow(unused_variables)]
#![allow(dead_code)]



pub mod blocks;
pub mod sstable;
pub mod memtable;
pub mod util;

pub mod iterator;

pub mod db;
use anyhow::Result;
use crate::db::DB;
pub fn open(path:&str) -> Result<Box<dyn db::DB>>{
    let db_impl = db::DBImpl::new();
    match db_impl.open(path) {
        Ok(_) => {
            return Ok(Box::new(db_impl));
        },
        Err(e) =>{
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let ret = open("");
    }
}
