use ipc::memory::MemorySource;
use table::RowBatch;
use ty::RawSchemaPtr;

pub enum RowBatchReader {}

extern "C" {
  pub fn write_row_batch(src: *const MemorySource, batch: *const RowBatch, pos: i64, recur_depth: i32) -> i64;
  pub fn get_row_batch_size(batch: *const RowBatch) -> i64;
  pub fn open_row_batch_reader(src: *const MemorySource, pos: i64) -> *const RowBatchReader;
  pub fn release_row_batch_reader(reader: *const RowBatchReader);
  pub fn get_row_batch(reader: *const RowBatchReader, schema: RawSchemaPtr) -> *const RowBatch;
}