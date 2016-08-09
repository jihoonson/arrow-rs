use common::status::{RawStatusPtr, Status};
use libc;

pub enum MemoryPool {}

// TODO: add safe APIs

extern "C" {
  pub fn mem_alloc(pool: *mut MemoryPool, buffer: *mut u8, size: i64) -> RawStatusPtr;
  pub fn mem_free(pool: *mut MemoryPool, buffer: *mut u8, size: i64);
  pub fn num_bytes_alloc(pool: *const MemoryPool) -> i64;
  pub fn default_mem_pool() -> *mut MemoryPool;
}