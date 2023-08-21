use crate::blocks::{BlockBuilder, Blocks};
use crate::sstable::{BlockPointer, Footer, IndexBlockRecord, SSTable};
use crate::util::env::{get_global_sequence_number, sstfile_path, FileObject};
use anyhow::Result;

pub const SSTABLE_SIZE_LIMIT: usize = 4 * 1024 * 1024; // 4MB
pub struct SSTableBuilder {
    data_blocks: Vec<Blocks>,
    block_builder: BlockBuilder,
}

impl SSTableBuilder {
    pub fn new() -> Self {
        SSTableBuilder {
            data_blocks: Vec::new(),
            block_builder: BlockBuilder::new(),
        }
    }
    pub fn approximate_size_after_add(&self, key: &[u8], value: &[u8]) -> usize {
        todo!()
    }
    // TODO(summerxwu): Maybe need a return value to indicate the result
    pub fn add(&mut self, key: &[u8], value: &[u8]) {
        if self.approximate_size_after_add(key, value) <= SSTABLE_SIZE_LIMIT
            && self.block_builder.add(key, value).is_ok()
        {
            return;
        }
        // finish current data_block
        let data_block_holder = self.block_builder.build();
        self.data_blocks.push(data_block_holder);
        self.block_builder.clean_up();
        // Add the failed KV pair agine
        // panic if failed
        self.block_builder
            .add(key, value)
            .expect("The build has already been reset, it should not failed to add content");
    }
    /// build will return the `SSTable` object and serializable the content to disk file
    pub fn build(&self) -> Result<SSTable> {
        let seq = get_global_sequence_number();
        let mut file_obj = FileObject::create(sstfile_path(seq).as_str())?;

        let mut indexes_records: Vec<IndexBlockRecord> = Vec::new();
        let mut offset_counter = 0;
        // Write data portion of SSTable
        for data_block in self.data_blocks {
            let buf = data_block.encode();

            let data_block_pointer = BlockPointer(offset_counter, buf.len());
            let largest_key = data_block.largest_key().to_vec();
            let item = IndexBlockRecord {
                largest_key,
                data_block_pointer,
            };
            indexes_records.push(item);

            file_obj.write(buf.as_ref())?;
            offset_counter = offset_counter + buf.len();
        }


        // Write index portion of SSTable
        for indexes_record in indexes_records {
            file_obj.write(indexes_record.encode().as_ref())?;
        }
        // Write Footer
        let footer = Footer{
            index_block_pointers: indexes_records,
            num_of_index_block: indexes_records.len(),
        }

        todo!()
    }
    fn evaluate_sstable_size(&self) -> usize {
        todo!()
    }
}
