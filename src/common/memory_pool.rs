use common::status::{RawStatusPtr, ArrowError};
use libc;
use std::ptr;

#[macro_use]
use common::status;

pub enum RawMemoryPool {}

pub type RawMemoryPoolMutPtr = *mut RawMemoryPool;

pub struct MemoryPool {
  pool: RawMemoryPoolMutPtr
}

impl MemoryPool {
  pub fn default() -> MemoryPool {
    MemoryPool {
      pool: unsafe { default_mem_pool() }
    }
  }

  pub fn raw_memory_pool(&self) -> RawMemoryPoolMutPtr {
    self.pool
  }
  
  pub fn alloc(&mut self, size: i64) -> Result<*mut u8, ArrowError> {
    unsafe {
      let buf: *mut u8 = ptr::null_mut();
      let s = mem_alloc(self.pool, buf, size);
      result_from_status!(s, buf)
    }
  }

  pub fn free(&mut self, buf: *mut u8, size: i64) {
    unsafe { mem_free(self.pool, buf, size) }
  }

  pub fn len(&self) -> i64 {
    unsafe { num_bytes_alloc(self.pool) }
  }
}

extern "C" {
  pub fn mem_alloc(pool: RawMemoryPoolMutPtr, buffer: *mut u8, size: i64) -> RawStatusPtr;
  pub fn mem_free(pool: RawMemoryPoolMutPtr, buffer: *mut u8, size: i64);
  pub fn num_bytes_alloc(pool: *const RawMemoryPool) -> i64;
  pub fn default_mem_pool() -> RawMemoryPoolMutPtr;
}