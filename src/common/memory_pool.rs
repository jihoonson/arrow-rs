use common::status;
use common::status::{RawStatusPtr, ArrowError};
use libc;

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
  
  pub fn alloc(&mut self, buffer: *mut u8, size: i64) -> Result<*mut u8, ArrowError> {
    unsafe {
      let s = mem_alloc(self.pool, buffer, size);
      if status::ok(s) {
        Ok(buffer)
      } else {
        Err(ArrowError::new(s))
      }
    }
  }
}

extern "C" {
  pub fn mem_alloc(pool: RawMemoryPoolMutPtr, buffer: *mut u8, size: i64) -> RawStatusPtr;
  pub fn mem_free(pool: RawMemoryPoolMutPtr, buffer: *mut u8, size: i64);
  pub fn num_bytes_alloc(pool: *const RawMemoryPool) -> i64;
  pub fn default_mem_pool() -> RawMemoryPoolMutPtr;
}