use common::memory_pool::{MemoryPool, RawMemoryPoolMutPtr};
use common::status::{RawStatusPtr, ArrowError};

#[macro_use]
use common::status;

pub trait Buffer {
  fn capacity(&self) -> i64;
  fn size(&self) -> i64;
  fn data(&self) -> *const u8;
}

pub trait Mutable {
  fn mut_data(&self) -> *mut u8;
  fn as_immut(&self) -> Buf;
}

pub trait Resizable<T=Self> {
  fn resize(&mut self, size: i64) -> Result<&mut T, ArrowError>;
  fn reserve(&mut self, capacity: i64) -> Result<&mut T, ArrowError>;
}

pub enum BufType {
  BASE,
  POOL  // resizable and mutable
}

macro_rules! impl_buffer {
  ($name:ident, $ty:ident) => (
    impl Buffer for $name {
      fn capacity(&self) -> i64 {
        unsafe {
          buf_capa(self.raw_buf)
        }
      }

      fn size(&self) -> i64 {
        unsafe {
          buf_size(self.raw_buf)
        }
      }

      fn data(&self) -> *const u8 {
        unsafe {
          buf_data(self.raw_buf)
        }
      }
    }

    impl PartialEq for $name {
      fn eq(&self, other: &$name) -> bool {
        unsafe {
          buf_equals(self.raw_buf, other.raw_buf)
        }
      }
    }

    impl Drop for $name {
      fn drop(&mut self) {
        unsafe {
          release_buf(self.raw_buf);
        }
      }
    }
  );
}

pub struct Buf {
  raw_buf: RawBufferPtr,
  ty: BufType
}

pub struct MutableBuf {
  raw_buf: RawBufferMutPtr,
  ty: BufType
}

impl_buffer!(Buf, BASE);
impl_buffer!(MutableBuf, BASE);

impl Buf {
  pub fn from_raw(raw_buf: RawBufferPtr) -> Buf {
    Buf {
      raw_buf: raw_buf,
      ty: BufType::BASE
    }
  }

  fn raw_buf(&self) -> RawBufferPtr {
    self.raw_buf
  }
}

impl MutableBuf {
  pub fn from_raw(raw_buf: RawBufferMutPtr) -> MutableBuf {
    MutableBuf {
      raw_buf: raw_buf,
      ty: BufType::POOL
    }
  }

  fn raw_buf(&self) -> RawBufferMutPtr {
    self.raw_buf
  }
}

impl Mutable for MutableBuf {
  fn mut_data(&self) -> *mut u8 {
    unsafe { buf_mut_data(self.raw_buf) }
  }

  fn as_immut(&self) -> Buf {
    Buf::from_raw( unsafe { buf_immut_view(self.raw_buf) } )
  }
}

impl Resizable for MutableBuf {
  fn resize(&mut self, size: i64) -> Result<&mut MutableBuf, ArrowError> {
    let s = unsafe { resize_buf(self.raw_buf, size) };
    result_from_status!(s, self)
  }

  fn reserve(&mut self, capacity: i64) -> Result<&mut MutableBuf, ArrowError> {
    let s = unsafe { reserve_buf(self.raw_buf, capacity) };
    result_from_status!(s, self)
  }
}

pub struct BufferBuilder {
  raw_builder: RawbufferBuilderMutPtr
}

impl BufferBuilder {
  pub fn new(pool: &MemoryPool) -> BufferBuilder {
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
    }
  }

  pub fn raw_append(&mut self, data: *const u8, len: i32) -> Result<i32, ArrowError> {
    unsafe {
      let s = raw_append_buf_builder(self.raw_builder, data, len);
      result_from_status!(s, buf_builder_len(self.raw_builder))
    }
  }

  pub fn finish(&mut self) -> MutableBuf {
    MutableBuf::from_raw( unsafe { finish_buf_builder(self.raw_builder) } )
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