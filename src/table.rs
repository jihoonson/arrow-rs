use ty;
use ty::{Schema};
use common::status;
use common::status::{RawStatusPtr, ArrowError};
use column::{RawColumnPtr, Column};
use array::{RawArrayPtr, Array};
use ipc::adapter::c_api::get_row_batch_size;
use libc;
use std::ffi::{CStr, CString};
use std::any::Any;

#[macro_use]
use common;

pub enum RecordBatch {}

pub struct RowBatch {
  raw_batch: RawRowBatchPtr
}

impl RowBatch {
  pub fn new<T: Any + Array>(schema: &Schema, num_rows: i32, arrays: &[T]) -> RowBatch {
    let raw_arrays = arrays.into_iter().map(|array| array.raw_array()).collect::<Vec<RawArrayPtr>>();
    RowBatch {
      raw_batch: unsafe { new_row_batch(schema.raw_schema(), num_rows, &raw_arrays, arrays.len() as i32)}
    }
  }

  pub fn from_raw(raw_batch: RawRowBatchPtr) -> RowBatch {
    RowBatch {
      raw_batch: raw_batch
    }
  }

  pub fn raw_batch(&self) -> RawRowBatchPtr {
    self.raw_batch
  }

  pub fn schema(&self) -> Schema {
    Schema::from_raw( unsafe { row_batch_schema(self.raw_batch) } )
  }

  pub fn column<T: Any + Array>(&self, i: i32) -> T {
    T::from_raw( unsafe { row_batch_column(self.raw_batch, i) } )
  }

  pub fn column_name(&self, i: i32) -> String {
    cstr_to_string!( unsafe { row_batch_col_name(self.raw_batch, i) } )
  }

  pub fn column_num(&self) -> i32 {
    unsafe { row_batch_num_cols(self.raw_batch) }
  }

  pub fn row_num(&self) -> i32 {
    unsafe { row_batch_num_rows(self.raw_batch) }
  }

  pub fn size(&self) -> i64 {
    unsafe { get_row_batch_size(self.raw_batch) }
  }
}

impl Drop for RowBatch {
  fn drop(&mut self) {
    unsafe { release_row_batch(self.raw_batch) }
  }
}

pub struct Table {
  raw_table: RawTablePtr
}

impl Table {
  pub fn new(name: String, schema: &Schema, columns: &[Column]) -> Table {
    let raw_cols = columns.into_iter().map(|col| col.raw_column()).collect::<Vec<RawColumnPtr>>();
    Table {
      raw_table: unsafe { new_table(string_to_cstr!(name), schema.raw_schema(), &raw_cols, columns.len() as i32)}
    }
  }

  pub fn name(&self) -> String {
    unsafe { cstr_to_string!(table_name(self.raw_table)) }
  }

  pub fn schema(&self) -> Schema {
    Schema::from_raw( unsafe { table_schema(self.raw_table) } )
  }

  pub fn column(&self, i: i32) -> Column {
    Column::from_raw( unsafe { table_column(self.raw_table, i) })
  }

  pub fn column_num(&self) -> i32 {
    unsafe { table_num_cols(self.raw_table) }
  }

  pub fn row_num(&self) -> i64 {
    unsafe { table_num_rows(self.raw_table) }
  }

  pub fn validate_columns(&self) -> Result<&Table, ArrowError> {
    let s = unsafe { validate_table_cols(self.raw_table) };
    result_from_status!(s, self)
  }
}

impl Drop for Table {
  fn drop(&mut self) {
    unsafe { release_table(self.raw_table) }
  }
}

pub enum RawRowBatch {}
pub enum RawTable {}

pub type RawRowBatchPtr = *const RawRowBatch;
pub type RawTablePtr = *const RawTable;

extern "C" {
  // RowBatch
  pub fn new_row_batch(schema: ty::RawSchemaPtr, num_rows: i32, arrays: &[RawArrayPtr], arr_len: i32) -> *const RawRowBatch;
  pub fn release_row_batch(row_batch: *const RawRowBatch);
  pub fn row_batch_schema(row_batch: *const RawRowBatch) -> ty::RawSchemaPtr;
  pub fn row_batch_column(row_batch: *const RawRowBatch, i: i32) -> RawArrayPtr;
  pub fn row_batch_col_name(row_batch: *const RawRowBatch, i: i32) -> *const libc::c_char;
  pub fn row_batch_num_cols(row_batch: *const RawRowBatch) -> i32;
  pub fn row_batch_num_rows(row_batch: *const RawRowBatch) -> i32;

  // Table
  pub fn new_table(name: *const libc::c_char, schema: ty::RawSchemaPtr, cols: &[RawColumnPtr], num_cols: i32) -> *const RawTable;
  pub fn release_table(table: *const RawTable);
  pub fn table_name(table: *const RawTable) -> *const libc::c_char;
  pub fn table_schema(table: *const RawTable) -> ty::RawSchemaPtr;
  pub fn table_column(table: *const RawTable, i: i32) -> RawColumnPtr;
  pub fn table_num_cols(table: *const RawTable) -> i32;
  pub fn table_num_rows(table: *const RawTable) -> i64;
  pub fn validate_table_cols(table: *const RawTable) -> RawStatusPtr;
}