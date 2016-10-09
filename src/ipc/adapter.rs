use io::memory::{MemoryMappedSource, MemorySource};
use table::RowBatch;
use ty::Schema;
use common::status::{ArrowError, RawStatusPtr};
use common::status;

use std::mem;

//pub struct RowBatchWriter {
//  mem_src: RawMemoryMappedSourceMutPtr,
//  start: i64,
//  pos: i64
//}
//
//impl RowBatchWriter {
//  pub fn open(src: MemoryMappedSource, pos: i64) -> RowBatchWriter {
//    RowBatchWriter {
//      mem_src: src.raw_source(),
//      start: pos,
//      pos: pos
//    }
//  }
//
//  pub fn write(batch: RowBatch) -> i64 {
//
//  }
//}

pub fn write_row_batch(src: &MemoryMappedSource, batch: &RowBatch, pos: i64) -> i64 {
  unsafe { c_api::write_row_batch(src.raw_source(), batch.raw_batch(), pos, 64) }
}

pub struct RowBatchReader {
  raw_reader: c_api::RawRowBatchReaderPtr
}

impl RowBatchReader {
  pub fn open(src: &MemoryMappedSource, pos: i64) -> Result<RowBatchReader, ArrowError> {
    unsafe {
      let raw_arrow_result = c_api::open_row_batch_reader(src.raw_source(), pos);
      let raw_status = (*raw_arrow_result).status;
      let raw_result = (*raw_arrow_result).result;
      c_api::release_arrow_result(raw_arrow_result);

      if status::ok(raw_status) {
        Ok(
          RowBatchReader {
            raw_reader: mem::transmute(raw_result)
          }
        )
      } else {
        Err(ArrowError::new(raw_status))
      }
    }
  }

  pub fn read(&self, schema: &Schema) -> RowBatch {
    RowBatch::from_raw( unsafe { c_api::get_row_batch(self.raw_reader, schema.raw_schema()) } )
  }
}

impl Drop for RowBatchReader {
  fn drop(&mut self) {
    unsafe { c_api::release_row_batch_reader(self.raw_reader) }
  }
}

#[repr(C)]
pub struct RawArrowResult {
  result: *const u8,
  status: RawStatusPtr
}

impl RawArrowResult {
  pub fn result(&self) -> *const u8 {
    self.result
  }

  pub fn status(&self) -> RawStatusPtr {
    self.status
  }
}

pub type RawArrowResultPtr = *const RawArrowResult;

pub mod c_api {
  use io::memory::RawMemoryMappedSourceMutPtr;
  use table::RawRowBatchPtr;
  use ty::RawSchemaPtr;

  use super::RawArrowResultPtr;

  pub enum RawRowBatchReader {}

  pub type RawRowBatchReaderPtr = *const RawRowBatchReader;

  extern "C" {
    pub fn write_row_batch(src: RawMemoryMappedSourceMutPtr, batch: RawRowBatchPtr, pos: i64, recur_depth: i32) -> i64;
    pub fn get_row_batch_size(batch: RawRowBatchPtr) -> i64;

    pub fn open_row_batch_reader(src: RawMemoryMappedSourceMutPtr, pos: i64) -> RawArrowResultPtr;
    pub fn release_row_batch_reader(reader: RawRowBatchReaderPtr);
    pub fn get_row_batch(reader: RawRowBatchReaderPtr, schema: RawSchemaPtr) -> RawRowBatchPtr;

    pub fn release_arrow_result(result: RawArrowResultPtr);
  }
}