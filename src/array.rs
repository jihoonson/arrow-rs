use ty;
use buffer::Buffer;

pub enum Array {}

extern "C" {
  pub fn arr_is_null(arr: *const Array, i: i32) -> bool;
  pub fn arr_len(arr: *const Array) -> i32;
  pub fn arr_null_count(arr: *const Array) -> i32;
  pub fn arr_type(arr: *const Array) -> *const ty::DataType;
  pub fn arr_type_enum(arr: *const Array) -> ty::Ty;
  pub fn arr_equals_exact(arr1: *const Array, arr2: *const Array) -> bool;
  pub fn arr_equals(arr1: *const Array, arr2: *const Array) -> bool;
  pub fn arr_range_equals(arr1: *const Array, arr2: *const Array, start: i32, end: i32, other_start: i32) -> bool;
  pub fn new_null_arr(ty: *const ty::DataType, length: i32) -> *const Array;
  pub fn release_arr(arr: *const Array);
}