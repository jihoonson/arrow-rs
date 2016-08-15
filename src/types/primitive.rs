use array::*;
use buffer::{RawBufferPtr, Buffer};
use ty::{RawDataTypePtr, DataType, Ty};
use ty;
use common::memory_pool::{RawMemoryPoolMutPtr, MemoryPool};
use common::status::{RawStatusPtr, ArrowError};
#[macro_use]
use common::status;
use std::mem;

pub trait PrimitiveArray<T, Ty=Self> : Array<Ty> {
  fn raw_data(&self) -> *const T;

  fn value(&self, i: i32) -> T;
}

macro_rules! define_raw_builder {
  ($name:ident) => (pub enum $name {});
}

define_raw_builder!(RawU8ArrayBuilder);
define_raw_builder!(RawI8ArrayBuilder);
define_raw_builder!(RawU16ArrayBuilder);
define_raw_builder!(RawI16ArrayBuilder);
define_raw_builder!(RawU32ArrayBuilder);
define_raw_builder!(RawI32ArrayBuilder);
define_raw_builder!(RawU64ArrayBuilder);
define_raw_builder!(RawI64ArrayBuilder);
define_raw_builder!(RawF32ArrayBuilder);
define_raw_builder!(RawF64ArrayBuilder);

macro_rules! define_array {
  ($name:ident, $ty:ident) => (
    pub struct $name {
      raw_array: RawArrayPtr
    }

    impl PrimitiveArray<$ty> for $name {
      fn raw_data(&self) -> *const $ty {
          unsafe { concat_idents!($ty, _arr_raw_data) (self.raw_array) }
      }

      fn value(&self, i: i32) -> $ty {
        unsafe { concat_idents!($ty, _arr_value) (self.raw_array, i) }
      }
    }

    impl PartialEq for $name {
      fn eq(&self, other: &$name) -> bool {
        unsafe { arr_equals_exact(self.raw_array, other.raw_array) }
      }
    }

    impl Drop for $name {
      fn drop(&mut self) {
        unsafe { release_arr(self.raw_array) }
      }
    }

    impl Array for $name {
      fn reinterpret(array: &BaseArray) -> &$name {
        unsafe { mem::transmute(array) }
      }

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

      fn ty(&self) -> Ty {
        unsafe { arr_type_enum(self.raw_array) }
      }

      fn range_equals(&self, other: &$name, start: i32, end: i32, other_start: i32) -> bool {
        unsafe { arr_range_equals(self.raw_array, other.raw_array, start, end, other_start) }
      }

      fn new_null_array(length: i32) -> $name {
        $name {
          raw_array: unsafe { new_null_arr(ty::new_primitive_type(Ty::UINT8), length) }
        }
      }

      fn raw_array(&self) -> RawArrayPtr {
        self.raw_array
      }

      fn data(&self) -> Buffer {
        Buffer::from_raw( unsafe { arr_data(self.raw_array) } )
      }
    }
  );
}

define_array!(U8Array, u8);
define_array!(I8Array, i8);
define_array!(U16Array, u16);
define_array!(I16Array, i16);
define_array!(U32Array, u32);
define_array!(I32Array, i32);
define_array!(U64Array, u64);
define_array!(I64Array, i64);
define_array!(F32Array, f32);
define_array!(F64Array, f64);

macro_rules! define_array_builder {
  ($name:ident) => (pub enum $name {});
}

pub struct U8ArrayBuilder {
  raw_builder: *mut RawU8ArrayBuilder
}

impl U8ArrayBuilder {
  pub fn new(pool: MemoryPool, data_type: DataType) -> U8ArrayBuilder {
    U8ArrayBuilder {
      raw_builder: unsafe { new_u8_arr_builder(pool.raw_memory_pool(), data_type.raw_data_type()) }
    }
  }

  pub fn init(&mut self, capacity: i32) -> Result<&mut U8ArrayBuilder, ArrowError> {
    unsafe {
      let s = init_u8_arr_builder(self.raw_builder, capacity);
      result_from_status!(s, self)
    }
  }

  pub fn append(&mut self, values: *const u8, len: i32, valid_bytes: *const u8) -> Result<&mut U8ArrayBuilder, ArrowError> {
    unsafe {
      let s = append_u8_arr_builder(self.raw_builder, values, len, valid_bytes);
      result_from_status!(s, self)
    }
  }

  pub fn finish(&mut self) -> U8Array {
    U8Array {
      raw_array: unsafe { finish_u8_arr_builder(self.raw_builder) }
    }
  }
}

extern "C" {
  pub fn arr_data(arr: RawArrayPtr) -> RawBufferPtr;

  pub fn u8_arr_raw_data(arr: RawArrayPtr) -> *const u8;
  pub fn i8_arr_raw_data(arr: RawArrayPtr) -> *const i8;
  pub fn u16_arr_raw_data(arr: RawArrayPtr) -> *const u16;
  pub fn i16_arr_raw_data(arr: RawArrayPtr) -> *const i16;
  pub fn u32_arr_raw_data(arr: RawArrayPtr) -> *const u32;
  pub fn i32_arr_raw_data(arr: RawArrayPtr) -> *const i32;
  pub fn u64_arr_raw_data(arr: RawArrayPtr) -> *const u64;
  pub fn i64_arr_raw_data(arr: RawArrayPtr) -> *const i64;
  pub fn f32_arr_raw_data(arr: RawArrayPtr) -> *const f32;
  pub fn f64_arr_raw_data(arr: RawArrayPtr) -> *const f64;

  pub fn u8_arr_value(arr: RawArrayPtr, i: i32) -> u8;
  pub fn i8_arr_value(arr: RawArrayPtr, i: i32) -> i8;
  pub fn u16_arr_value(arr: RawArrayPtr, i: i32) -> u16;
  pub fn i16_arr_value(arr: RawArrayPtr, i: i32) -> i16;
  pub fn u32_arr_value(arr: RawArrayPtr, i: i32) -> u32;
  pub fn i32_arr_value(arr: RawArrayPtr, i: i32) -> i32;
  pub fn u64_arr_value(arr: RawArrayPtr, i: i32) -> u64;
  pub fn i64_arr_value(arr: RawArrayPtr, i: i32) -> i64;
  pub fn f32_arr_value(arr: RawArrayPtr, i: i32) -> f32;
  pub fn f64_arr_value(arr: RawArrayPtr, i: i32) -> f64;

  pub fn new_u8_arr_builder(pool: RawMemoryPoolMutPtr, ty: RawDataTypePtr) -> *mut RawU8ArrayBuilder;
  pub fn new_i8_arr_builder(pool: RawMemoryPoolMutPtr, ty: RawDataTypePtr) -> *mut RawI8ArrayBuilder;
  pub fn new_u16_arr_builder(pool: RawMemoryPoolMutPtr, ty: RawDataTypePtr) -> *mut RawU16ArrayBuilder;
  pub fn new_i16_arr_builder(pool: RawMemoryPoolMutPtr, ty: RawDataTypePtr) -> *mut RawI16ArrayBuilder;
  pub fn new_u32_arr_builder(pool: RawMemoryPoolMutPtr, ty: RawDataTypePtr) -> *mut RawU32ArrayBuilder;
  pub fn new_i32_arr_builder(pool: RawMemoryPoolMutPtr, ty: RawDataTypePtr) -> *mut RawI32ArrayBuilder;
  pub fn new_u64_arr_builder(pool: RawMemoryPoolMutPtr, ty: RawDataTypePtr) -> *mut RawU64ArrayBuilder;
  pub fn new_i64_arr_builder(pool: RawMemoryPoolMutPtr, ty: RawDataTypePtr) -> *mut RawI64ArrayBuilder;
  pub fn new_f32_arr_builder(pool: RawMemoryPoolMutPtr, ty: RawDataTypePtr) -> *mut RawF32ArrayBuilder;
  pub fn new_f64_arr_builder(pool: RawMemoryPoolMutPtr, ty: RawDataTypePtr) -> *mut RawF64ArrayBuilder;

  pub fn init_u8_arr_builder(builder: *mut RawU8ArrayBuilder, capa: i32) -> RawStatusPtr;
  pub fn init_i8_arr_builder(builder: *mut RawI8ArrayBuilder, capa: i32) -> RawStatusPtr;
  pub fn init_u16_arr_builder(builder: *mut RawU16ArrayBuilder, capa: i32) -> RawStatusPtr;
  pub fn init_i16_arr_builder(builder: *mut RawI16ArrayBuilder, capa: i32) -> RawStatusPtr;
  pub fn init_u32_arr_builder(builder: *mut RawU32ArrayBuilder, capa: i32) -> RawStatusPtr;
  pub fn init_i32_arr_builder(builder: *mut RawI32ArrayBuilder, capa: i32) -> RawStatusPtr;
  pub fn init_u64_arr_builder(builder: *mut RawU64ArrayBuilder, capa: i32) -> RawStatusPtr;
  pub fn init_i64_arr_builder(builder: *mut RawI64ArrayBuilder, capa: i32) -> RawStatusPtr;
  pub fn init_f32_arr_builder(builder: *mut RawF32ArrayBuilder, capa: i32) -> RawStatusPtr;
  pub fn init_f64_arr_builder(builder: *mut RawF64ArrayBuilder, capa: i32) -> RawStatusPtr;

  pub fn append_u8_arr_builder(builder: *mut RawU8ArrayBuilder, values: *const u8, len: i32, valid_bytes: *const u8) -> RawStatusPtr;
  pub fn append_i8_arr_builder(builder: *mut RawI8ArrayBuilder, values: *const i8, len: i32, valid_bytes: *const u8) -> RawStatusPtr;
  pub fn append_u16_arr_builder(builder: *mut RawU16ArrayBuilder, values: *const u16, len: i32, valid_bytes: *const u8) -> RawStatusPtr;
  pub fn append_i16_arr_builder(builder: *mut RawI16ArrayBuilder, values: *const i16, len: i32, valid_bytes: *const u8) -> RawStatusPtr;
  pub fn append_u32_arr_builder(builder: *mut RawU32ArrayBuilder, values: *const u32, len: i32, valid_bytes: *const u8) -> RawStatusPtr;
  pub fn append_i32_arr_builder(builder: *mut RawI32ArrayBuilder, values: *const i32, len: i32, valid_bytes: *const u8) -> RawStatusPtr;
  pub fn append_u64_arr_builder(builder: *mut RawU64ArrayBuilder, values: *const u64, len: i32, valid_bytes: *const u8) -> RawStatusPtr;
  pub fn append_i64_arr_builder(builder: *mut RawI64ArrayBuilder, values: *const i64, len: i32, valid_bytes: *const u8) -> RawStatusPtr;
  pub fn append_f32_arr_builder(builder: *mut RawF32ArrayBuilder, values: *const f32, len: i32, valid_bytes: *const u8) -> RawStatusPtr;
  pub fn append_f64_arr_builder(builder: *mut RawF64ArrayBuilder, values: *const f64, len: i32, valid_bytes: *const u8) -> RawStatusPtr;

  pub fn finish_u8_arr_builder(builder: *mut RawU8ArrayBuilder) -> RawArrayPtr;
  pub fn finish_i8_arr_builder(builder: *mut RawI8ArrayBuilder) -> RawArrayPtr;
  pub fn finish_u16_arr_builder(builder: *mut RawU16ArrayBuilder) -> RawArrayPtr;
  pub fn finish_i16_arr_builder(builder: *mut RawI16ArrayBuilder) -> RawArrayPtr;
  pub fn finish_u32_arr_builder(builder: *mut RawU32ArrayBuilder) -> RawArrayPtr;
  pub fn finish_i32_arr_builder(builder: *mut RawI32ArrayBuilder) -> RawArrayPtr;
  pub fn finish_u64_arr_builder(builder: *mut RawU64ArrayBuilder) -> RawArrayPtr;
  pub fn finish_i64_arr_builder(builder: *mut RawI64ArrayBuilder) -> RawArrayPtr;
  pub fn finish_f32_arr_builder(builder: *mut RawF32ArrayBuilder) -> RawArrayPtr;
  pub fn finish_f64_arr_builder(builder: *mut RawF64ArrayBuilder) -> RawArrayPtr;
}
