use common::status::{RawStatusPtr, ArrowError};
use libc;
use std::ptr;

#[macro_use]
use common::status;
use common::status::Status;

cxx_inline! {
  #include "arrow/util/memory-pool.h"
  #include "arrow/util/status.h"

  using namespace arrow;
}

pub enum RawMemoryPool {}

pub type RawMemoryPoolMutPtr = *mut RawMemoryPool;

pub enum MemoryPool {}

pub struct MemoryPoolWrapper {
  pool: *mut MemoryPool
}

impl MemoryPoolWrapper {
  pub fn default() -> MemoryPoolWrapper {
    MemoryPoolWrapper {
      pool: unsafe { cxx![() -> *mut MemoryPool { default_memory_pool() }] }
    }
  }

  pub fn raw_memory_pool(&self) -> *mut MemoryPool {
    self.pool
  }
  
  pub fn alloc(&mut self, size: i64) -> Result<*mut u8, ArrowError> {
    unsafe {
      let buf: *mut u8 = ptr::null_mut();
//      let s = mem_alloc(self.pool, buf, size);
      let s = unsafe {
        cxx![(pool: *mut MemoryPool = self.pool, size: i64, buf: *mut u8) -> Status {
          pool->Allocate(size, const_cast<uint8_t **>(&buf))
        }]
      };
      result_from_status!(s, buf)
    }
  }

  pub fn free(&mut self, buf: *mut u8, size: i64) {
//    unsafe { mem_free(self.pool, buf, size) }
    unsafe { cxx![(pool: *mut MemoryPool = self.pool, size: i64, buf: *mut u8) { pool->Free(buf, size); }] }
  }

  pub fn len(&self) -> i64 {
//    unsafe { num_bytes_alloc(self.pool) }
    unsafe { cxx![ (pool: *mut MemoryPool = self.pool) -> i64 { pool->bytes_allocated() } ] }
  }
}

//extern "C" {
//  pub fn mem_alloc(pool: RawMemoryPoolMutPtr, buffer: *mut u8, size: i64) -> RawStatusPtr;
//  pub fn mem_free(pool: RawMemoryPoolMutPtr, buffer: *mut u8, size: i64);
//  pub fn num_bytes_alloc(pool: *const RawMemoryPool) -> i64;
//  pub fn default_mem_pool() -> RawMemoryPoolMutPtr;
//}