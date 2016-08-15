use common::memory_pool::{MemoryPool, RawMemoryPoolMutPtr};
use common::status::{RawStatusPtr, ArrowError};
#[macro_use]
use common::status;

pub struct Buffer {
  raw_buf: RawBufferPtr
}

pub struct BufferBuilder {
  raw_builder: RawbufferBuilderMutPtr
}

impl Buffer {
  pub fn from_raw(raw_buf: RawBufferPtr) -> Buffer {
    Buffer {
      raw_buf: raw_buf
    }
  }

  pub fn raw_buf(&self) -> RawBufferPtr {
    self.raw_buf
  }

  pub fn capacity(&self) -> i64 {
    unsafe {
      buf_capa(self.raw_buf)
    }
  }

  pub fn size(&self) -> i64 {
    unsafe {
      buf_size(self.raw_buf)
    }
  }

  pub fn data(&self) -> *const u8 {
    unsafe {
      buf_data(self.raw_buf)
    }
  }

  pub fn mut_data(&self) -> *mut u8 {
    unsafe {
      buf_mut_data(self.raw_buf)
    }
  }

  pub fn immutable_view(&self) -> Buffer {
    unsafe {
      Buffer {
        raw_buf: buf_immut_view(self.raw_buf)
      }
    }
  }
}

impl PartialEq for Buffer {
  fn eq(&self, other: &Buffer) -> bool {
    unsafe {
      buf_equals(self.raw_buf, other.raw_buf)
    }
  }
}

impl Drop for Buffer {
  fn drop(&mut self) {
    unsafe {
      release_buf(self.raw_buf);
    }
  }
}

impl BufferBuilder {
  pub fn new(pool: MemoryPool) -> BufferBuilder {
    unsafe {
      BufferBuilder {
        raw_builder: new_buf_builder(pool.raw_memory_pool())
      }
    }
  }

  pub fn resize(&mut self, size: i32) -> Result<i32, ArrowError> {
    unsafe {
      let s = resize_buf_builder(self.raw_builder, size);
      result_from_status!(s, buf_builder_capa(self.raw_builder))
//      if status::ok(s) {
//        status::release_status(s);
//        Ok(buf_builder_capa(self.raw_builder))
//      } else {
//        Err(ArrowError::new(s))
//      }
    }
  }

  pub fn raw_append(&mut self, data: *const u8, len: i32) -> Result<i32, ArrowError> {
    unsafe {
      let s = raw_append_buf_builder(self.raw_builder, data, len);
      result_from_status!(s, buf_builder_len(self.raw_builder))
//      if status::ok(s) {
//        status::release_status(s);
//        Ok(buf_builder_len(self.raw_builder))
//      } else {
//        Err(ArrowError::new(s))
//      }
    }
  }

  pub fn finish(&mut self) -> Buffer {
    unsafe {
      Buffer {
        raw_buf: finish_buf_builder(self.raw_builder)
      }
    }
  }

  pub fn capacity(&mut self) -> i32 {
    unsafe {
      buf_builder_capa(self.raw_builder)
    }
  }

  pub fn len(&mut self) -> i32 {
    unsafe {
      buf_builder_len(self.raw_builder)
    }
  }
}

impl Drop for BufferBuilder {
  fn drop(&mut self) {
    unsafe {
      release_buf_builder(self.raw_builder)
    }
  }
}

pub enum RawBuffer {}
pub enum RawBufferBuilder {}

pub type RawBufferPtr = *const RawBuffer;
pub type RawBufferMutPtr = *mut RawBuffer;
pub type RawbufferBuilderMutPtr = *const RawBufferBuilder;

extern "C" {
  // Buffer
  pub fn release_buf(buf: RawBufferPtr);

  pub fn buf_part_equals(buf1: RawBufferPtr, buf2: RawBufferPtr, nbytes: i64) -> bool;
  pub fn buf_equals(buf1: RawBufferPtr, buf2: RawBufferPtr) -> bool;
  pub fn buf_capa(buf: RawBufferPtr) -> i64;
  pub fn buf_size(buf: RawBufferPtr) -> i64;
  pub fn buf_data(buf: RawBufferPtr) -> *const u8;

  // Mutable buffer
  pub fn buf_mut_data(buf: RawBufferPtr) -> *mut u8;
  pub fn buf_immut_view(buf: RawBufferPtr) -> RawBufferPtr;

  // Pool buffer
  pub fn resize_buf(buf: RawBufferMutPtr, new_size: i64) -> RawStatusPtr;
  pub fn reserve_buf(buf: RawBufferMutPtr, new_capa: i64) -> RawStatusPtr;

  // Buffer builder
  pub fn new_buf_builder(pool: RawMemoryPoolMutPtr) -> RawbufferBuilderMutPtr;
  pub fn release_buf_builder(builder: RawbufferBuilderMutPtr);
  pub fn resize_buf_builder(builder: RawbufferBuilderMutPtr, elements: i32) -> RawStatusPtr;
  pub fn raw_append_buf_builder(builder: RawbufferBuilderMutPtr, data: *const u8, len: i32) -> RawStatusPtr;
  pub fn finish_buf_builder(builder: RawbufferBuilderMutPtr) -> RawBufferMutPtr;
  pub fn buf_builder_capa(builder: RawbufferBuilderMutPtr) -> i32;
  pub fn buf_builder_len(builder: RawbufferBuilderMutPtr) -> i32;
}