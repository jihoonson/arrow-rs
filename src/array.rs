use ty::{DataType, RawDataTypePtr, Ty};
use ty;
use buffer::{Buf, Buffer};
use types::primitive;

// TODO: inheritance relationship for Array and its sub classes

pub trait Array<Ty=Self> {
  fn reinterpret(array: &BaseArray) -> &Ty;

  fn is_null(&self, i: i32) -> bool;

  fn len(&self) -> i32;

  fn null_count(&self) -> i32;

  fn data_type(&self) -> DataType;

  fn ty(&self) -> ty::Ty;

  fn range_equals(&self, other: &Ty, start: i32, end: i32, other_start: i32) -> bool;

  fn raw_array(&self) -> RawArrayPtr;

  fn data(&self) -> Buf;

  fn new_null_array(length: i32) -> Ty;
}

pub struct BaseArray {
  raw_array: RawArrayPtr
}

impl BaseArray {
  pub fn from_raw(raw_array: RawArrayPtr) -> BaseArray {
    BaseArray {
      raw_array: raw_array
    }
  }

  pub fn raw_array(&self) -> RawArrayPtr {
    self.raw_array
  }
}

impl Array for BaseArray {
  fn is_null(&self, i: i32) -> bool {
    unsafe { arr_is_null(self.raw_array, i) }
  }

  fn len(&self) -> i32 {
    unsafe { arr_len(self.raw_array) }
  }

  fn null_count(&self) -> i32 {
    unsafe { arr_null_count(self.raw_array) }
  }

  fn data_type(&self) -> DataType {
    DataType::new(unsafe { arr_type(self.raw_array) })
  }

  fn ty(&self) -> ty::Ty {
    unsafe { arr_type_enum(self.raw_array) }
  }

  fn range_equals(&self, other: &BaseArray, start: i32, end: i32, other_start: i32) -> bool {
    unsafe { arr_range_equals(self.raw_array, other.raw_array, start, end, other_start) }
  }

  fn raw_array(&self) -> RawArrayPtr {
    self.raw_array
  }

  fn data(&self) -> Buf {
    Buf::from_raw( unsafe { primitive::arr_data(self.raw_array) } )
  }

  fn new_null_array(length: i32) -> BaseArray {
    // must not be called
    unimplemented!()
  }

  fn reinterpret(array: &BaseArray) -> &BaseArray {
    &array
  }
}

impl PartialEq for BaseArray {
  fn eq(&self, other: &BaseArray) -> bool {
    unsafe { arr_equals_exact(self.raw_array, other.raw_array) }
  }
}

impl Drop for BaseArray {
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
  pub fn arr_type_enum(arr: RawArrayPtr) -> ty::Ty;
  pub fn arr_equals_exact(arr1: RawArrayPtr, arr2: RawArrayPtr) -> bool;
  pub fn arr_equals(arr1: RawArrayPtr, arr2: RawArrayPtr) -> bool;
  pub fn arr_range_equals(arr1: RawArrayPtr, arr2: RawArrayPtr, start: i32, end: i32, other_start: i32) -> bool;
  pub fn new_null_arr(ty: RawDataTypePtr, length: i32) -> RawArrayPtr;
  pub fn release_arr(arr: RawArrayPtr);
}