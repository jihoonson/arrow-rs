use array::Array;
use ty;
use common::status::Status;

pub enum ChunkedArray {}
pub enum Column {}

extern "C" {
  // Chunked array
  pub fn new_chunked_arr(arrays: [Array], arr_len: i32) -> *const ChunkedArray;
  pub fn release_chunked_arr(chunked_arr: *const ChunkedArray);

  // Column
  pub fn new_column_from_arr(field: *const ty::Field, arr: *const Array) -> *const Column;
  pub fn new_column_from_chunked_arr(field: *const ty::Field, arr: *const ChunkedArray) -> *const Column;
  pub fn release_column(column: *const Column);
  pub fn column_len(column: *const Column) -> i64;
  pub fn column_null_count(column: *const Column) -> i64;
  pub fn column_type(column: *const Column) -> *const ty::DataType;
  pub fn column_data(column: *const Column) -> *const ChunkedArray;
  pub fn validate_column_data(column: *const Column) -> *const Status;
}