use array::Array;
use buffer::Buffer;
use ty::{RawDataTypePtr};
use common::memory_pool::MemoryPool;
use common::status::{RawStatusPtr, Status};

macro_rules! builder_decl {
  ($name:ident) => (pub enum $name {});
}

builder_decl!(U8ArrayBuilder);
builder_decl!(I8ArrayBuilder);
builder_decl!(U16ArrayBuilder);
builder_decl!(I16ArrayBuilder);
builder_decl!(U32ArrayBuilder);
builder_decl!(I32ArrayBuilder);
builder_decl!(U64ArrayBuilder);
builder_decl!(I64ArrayBuilder);
builder_decl!(F32ArrayBuilder);
builder_decl!(F64ArrayBuilder);

extern "C" {
  pub fn arr_data(arr: *const Array) -> *const Buffer;

  pub fn u8_arr_raw_data(arr: *const Array) -> *const u8;
  pub fn i8_arr_raw_data(arr: *const Array) -> *const i8;
  pub fn u16_arr_raw_data(arr: *const Array) -> *const u16;
  pub fn i16_arr_raw_data(arr: *const Array) -> *const i16;
  pub fn u32_arr_raw_data(arr: *const Array) -> *const u32;
  pub fn i32_arr_raw_data(arr: *const Array) -> *const i32;
  pub fn u64_arr_raw_data(arr: *const Array) -> *const u64;
  pub fn i64_arr_raw_data(arr: *const Array) -> *const i64;

  pub fn u8_arr_value(arr: *const Array, i: i32) -> u8;
  pub fn i8_arr_value(arr: *const Array, i: i32) -> i8;
  pub fn u16_arr_value(arr: *const Array, i: i32) -> u16;
  pub fn i16_arr_value(arr: *const Array, i: i32) -> i16;
  pub fn u32_arr_value(arr: *const Array, i: i32) -> u32;
  pub fn i32_arr_value(arr: *const Array, i: i32) -> i32;
  pub fn u64_arr_value(arr: *const Array, i: i32) -> u64;
  pub fn i64_arr_value(arr: *const Array, i: i32) -> i64;
  pub fn f32_arr_value(arr: *const Array, i: i32) -> f32;
  pub fn f64_arr_value(arr: *const Array, i: i32) -> f64;

  pub fn new_u8_arr_builder(pool: *mut MemoryPool, ty: RawDataTypePtr) -> *mut U8ArrayBuilder;
  pub fn new_i8_arr_builder(pool: *mut MemoryPool, ty: RawDataTypePtr) -> *mut I8ArrayBuilder;
  pub fn new_u16_arr_builder(pool: *mut MemoryPool, ty: RawDataTypePtr) -> *mut U16ArrayBuilder;
  pub fn new_i16_arr_builder(pool: *mut MemoryPool, ty: RawDataTypePtr) -> *mut I16ArrayBuilder;
  pub fn new_u32_arr_builder(pool: *mut MemoryPool, ty: RawDataTypePtr) -> *mut U32ArrayBuilder;
  pub fn new_i32_arr_builder(pool: *mut MemoryPool, ty: RawDataTypePtr) -> *mut I32ArrayBuilder;
  pub fn new_u64_arr_builder(pool: *mut MemoryPool, ty: RawDataTypePtr) -> *mut U64ArrayBuilder;
  pub fn new_i64_arr_builder(pool: *mut MemoryPool, ty: RawDataTypePtr) -> *mut I64ArrayBuilder;
  pub fn new_f32_arr_builder(pool: *mut MemoryPool, ty: RawDataTypePtr) -> *mut F32ArrayBuilder;
  pub fn new_f64_arr_builder(pool: *mut MemoryPool, ty: RawDataTypePtr) -> *mut F64ArrayBuilder;

  pub fn init_u8_arr_builder(builder: *mut U8ArrayBuilder, capa: i32) -> RawStatusPtr;
  pub fn init_i8_arr_builder(builder: *mut I8ArrayBuilder, capa: i32) -> RawStatusPtr;
  pub fn init_u16_arr_builder(builder: *mut U16ArrayBuilder, capa: i32) -> RawStatusPtr;
  pub fn init_i16_arr_builder(builder: *mut I16ArrayBuilder, capa: i32) -> RawStatusPtr;
  pub fn init_u32_arr_builder(builder: *mut U32ArrayBuilder, capa: i32) -> RawStatusPtr;
  pub fn init_i32_arr_builder(builder: *mut I32ArrayBuilder, capa: i32) -> RawStatusPtr;
  pub fn init_u64_arr_builder(builder: *mut U64ArrayBuilder, capa: i32) -> RawStatusPtr;
  pub fn init_i64_arr_builder(builder: *mut I64ArrayBuilder, capa: i32) -> RawStatusPtr;
  pub fn init_f32_arr_builder(builder: *mut F32ArrayBuilder, capa: i32) -> RawStatusPtr;
  pub fn init_f64_arr_builder(builder: *mut F64ArrayBuilder, capa: i32) -> RawStatusPtr;

  pub fn append_u8_arr_builder(builder: *mut U8ArrayBuilder, values: *const u8, len: i32, valid_bytes: *const u8) -> RawStatusPtr;
  pub fn append_i8_arr_builder(builder: *mut I8ArrayBuilder, values: *const i8, len: i32, valid_bytes: *const u8) -> RawStatusPtr;
  pub fn append_u16_arr_builder(builder: *mut U16ArrayBuilder, values: *const u16, len: i32, valid_bytes: *const u8) -> RawStatusPtr;
  pub fn append_i16_arr_builder(builder: *mut I16ArrayBuilder, values: *const i16, len: i32, valid_bytes: *const u8) -> RawStatusPtr;
  pub fn append_u32_arr_builder(builder: *mut U32ArrayBuilder, values: *const u32, len: i32, valid_bytes: *const u8) -> RawStatusPtr;
  pub fn append_i32_arr_builder(builder: *mut I32ArrayBuilder, values: *const i32, len: i32, valid_bytes: *const u8) -> RawStatusPtr;
  pub fn append_u64_arr_builder(builder: *mut U64ArrayBuilder, values: *const u64, len: i32, valid_bytes: *const u8) -> RawStatusPtr;
  pub fn append_i64_arr_builder(builder: *mut I64ArrayBuilder, values: *const i64, len: i32, valid_bytes: *const u8) -> RawStatusPtr;
  pub fn append_f32_arr_builder(builder: *mut F32ArrayBuilder, values: *const f32, len: i32, valid_bytes: *const u8) -> RawStatusPtr;
  pub fn append_f64_arr_builder(builder: *mut F64ArrayBuilder, values: *const f64, len: i32, valid_bytes: *const u8) -> RawStatusPtr;

  pub fn finish_u8_arr_builder(builder: *mut U8ArrayBuilder) -> *const Array;
  pub fn finish_i8_arr_builder(builder: *mut I8ArrayBuilder) -> *const Array;
  pub fn finish_u16_arr_builder(builder: *mut U16ArrayBuilder) -> *const Array;
  pub fn finish_i16_arr_builder(builder: *mut I16ArrayBuilder) -> *const Array;
  pub fn finish_u32_arr_builder(builder: *mut U32ArrayBuilder) -> *const Array;
  pub fn finish_i32_arr_builder(builder: *mut I32ArrayBuilder) -> *const Array;
  pub fn finish_u64_arr_builder(builder: *mut U64ArrayBuilder) -> *const Array;
  pub fn finish_i64_arr_builder(builder: *mut I64ArrayBuilder) -> *const Array;
  pub fn finish_f32_arr_builder(builder: *mut F32ArrayBuilder) -> *const Array;
  pub fn finish_f64_arr_builder(builder: *mut F64ArrayBuilder) -> *const Array;
}
