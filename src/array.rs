use ty::{Ty, DataType, RawDataTypePtr};

// TODO: inheritance relationship for Array and its sub classes

pub struct Array {
  raw_array: RawArrayPtr
}

impl Array {
  pub fn is_null(&self, i: i32) -> bool {
    unsafe { arr_is_null(self.raw_array, i) }
  }

  pub fn len(&self) -> i32 {
    unsafe { arr_len(self.raw_array) }
  }

  pub fn null_count(&self) -> i32 {
    unsafe { arr_null_count(self.raw_array) }
  }

  pub fn data_type(&self) -> DataType {
    DataType::new(unsafe { arr_type(self.raw_array) })
  }

  pub fn ty(&self) -> Ty {
    unsafe { arr_type_enum(self.raw_array) }
  }

  pub fn range_equals(&self, other: &Array, start: i32, end: i32, other_start: i32) -> bool {
    unsafe { arr_range_equals(self.raw_array, other.raw_array, start, end, other_start) }
  }

  pub fn new_null_array(data_type: DataType, length: i32) -> Array {
    Array {
      raw_array: unsafe { new_null_arr(data_type.raw_data_type(), length) }
    }
  }

  pub fn raw_array(&self) -> RawArrayPtr {
    self.raw_array
  }
}

impl PartialEq for Array {
  fn eq(&self, other: &Array) -> bool {
    unsafe { arr_equals(self.raw_array, other.raw_array) }
  }
}

impl Drop for Array {
  fn drop(&mut self) {
    unsafe { release_arr(self.raw_array) }
  }
}

pub enum RawArray {}

pub type RawArrayPtr = *const RawArray;

extern "C" {
  pub fn arr_is_null(arr: RawArrayPtr, i: i32) -> bool;
  pub fn arr_len(arr: RawArrayPtr) -> i32;
  pub fn arr_null_count(arr: RawArrayPtr) -> i32;
  pub fn arr_type(arr: RawArrayPtr) -> RawDataTypePtr;
  pub fn arr_type_enum(arr: RawArrayPtr) -> Ty;
  pub fn arr_equals_exact(arr1: RawArrayPtr, arr2: RawArrayPtr) -> bool;
  pub fn arr_equals(arr1: RawArrayPtr, arr2: RawArrayPtr) -> bool;
  pub fn arr_range_equals(arr1: RawArrayPtr, arr2: RawArrayPtr, start: i32, end: i32, other_start: i32) -> bool;
  pub fn new_null_arr(ty: RawDataTypePtr, length: i32) -> RawArrayPtr;
  pub fn release_arr(arr: RawArrayPtr);
}