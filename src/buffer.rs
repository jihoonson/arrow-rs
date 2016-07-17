use common::memory_pool::MemoryPool;
use common::status::Status;

pub enum Buffer {}
pub enum BufferBuilder {}

extern "C" {
  // Buffer
  pub fn release_buf(buf: *const Buffer);

  pub fn buf_part_equals(buf1: *const Buffer, buf2: *const Buffer, nbytes: i64) -> bool;
  pub fn buf_equals(buf1: *const Buffer, buf2: *const Buffer) -> bool;
  pub fn buf_capa(buf: *const Buffer) -> i64;
  pub fn buf_size(buf: *const Buffer) -> i64;
  pub fn buf_data(buf: *const Buffer) -> *const u8;

  // Mutable buffer
  pub fn buf_mut_data(buf: *const Buffer) -> *mut u8;
  pub fn buf_immut_view(buf: *const Buffer) -> *const Buffer;

  // Pool buffer
  pub fn resize_buf(buf: *mut Buffer, new_size: i64) -> *const Status;
  pub fn reserve_buf(buf: *mut Buffer, new_capa: i64) -> *const Status;

  // Buffer builder
  pub fn new_buf_builder(pool: *mut MemoryPool) -> *mut BufferBuilder;
  pub fn release_buf_builder(builder: *const BufferBuilder);
  pub fn resize_buf_builder(builder: *mut BufferBuilder, elements: i32) -> *const Status;
  pub fn raw_append_buf_builder(builder: *mut BufferBuilder, data: *const u8, len: i32) ->*const Status;
  pub fn finish_buf_builder(builder: *const BufferBuilder) -> *mut Buffer;
  pub fn buf_builder_capa(builder: *const BufferBuilder) -> i32;
  pub fn buf_builder_len(builder: *const BufferBuilder) -> i32;
}