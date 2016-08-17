#[macro_use]
use common::status;
use common::status::{RawStatusPtr, ArrowError};
use array::{RawArrayPtr, BaseArray};
use ty::{RawFieldPtr, RawDataTypePtr, Field, DataType};

pub struct ChunkedArray {
  raw_array: RawChunkedArrayPtr
}

pub struct Column {
  raw_column: RawColumnPtr
}

impl ChunkedArray {
  pub fn new(arrays: &[BaseArray]) -> ChunkedArray {
    let raw_arrays = arrays.into_iter().map(|each_array| each_array.raw_array()).collect::<Vec<RawArrayPtr>>();

    ChunkedArray {
      raw_array: unsafe { new_chunked_arr(&raw_arrays, arrays.len() as i32) }
    }
  }
}

impl Drop for ChunkedArray {
  fn drop(&mut self) {
    unsafe {
      release_chunked_arr(self.raw_array);
    }
  }
}

impl Column {
  pub fn from_raw(raw_column: RawColumnPtr) -> Column {
    Column {
      raw_column: raw_column
    }
  }

  pub fn from_array(field: &Field, array: &BaseArray) -> Column {
    Column {
      raw_column: unsafe { new_column_from_arr(field.raw_field(), array.raw_array()) }
    }
  }

  pub fn from_chunked_array(field: Field, array: ChunkedArray) -> Column {
    Column {
      raw_column: unsafe { new_column_from_chunked_arr(field.raw_field(), array.raw_array) }
    }
  }

  pub fn len(&self) -> i64 {
    unsafe { column_len(self.raw_column) }
  }

  pub fn null_count(&self) -> i64 {
    unsafe { column_null_count(self.raw_column) }
  }

  pub fn data_type(&self) -> DataType {
    DataType::new(unsafe { column_type(self.raw_column) })
  }

  pub fn data(&self) -> ChunkedArray {
    ChunkedArray {
      raw_array: unsafe { column_data(self.raw_column) }
    }
  }

  pub fn validate_data(&self) -> Result<ChunkedArray, ArrowError> {
    unsafe {
      let s = validate_column_data(self.raw_column);
      result_from_status!(s, self.data())
    }
  }

  pub fn raw_column(&self) -> RawColumnPtr {
    self.raw_column
  }
}

impl Drop for Column {
  fn drop(&mut self) {
    unsafe { release_column(self.raw_column) }
  }
}

pub enum RawChunkedArray {}
pub enum RawColumn {}

pub type RawChunkedArrayPtr = *const RawChunkedArray;
pub type RawColumnPtr = *const RawColumn;

extern "C" {
  // Chunked array
  pub fn new_chunked_arr(arrays: &[RawArrayPtr], arr_len: i32) -> RawChunkedArrayPtr;
  pub fn release_chunked_arr(chunked_arr: RawChunkedArrayPtr);

  // Column
  pub fn new_column_from_arr(field: RawFieldPtr, arr: RawArrayPtr) -> RawColumnPtr;
  pub fn new_column_from_chunked_arr(field: RawFieldPtr, arr: RawChunkedArrayPtr) -> RawColumnPtr;
  pub fn release_column(column: RawColumnPtr);
  pub fn column_len(column: RawColumnPtr) -> i64;
  pub fn column_null_count(column: RawColumnPtr) -> i64;
  pub fn column_type(column: RawColumnPtr) -> RawDataTypePtr;
  pub fn column_data(column: RawColumnPtr) -> RawChunkedArrayPtr;
  pub fn validate_column_data(column: RawColumnPtr) -> RawStatusPtr;
}