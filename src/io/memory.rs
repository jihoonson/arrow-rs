#[macro_use]
use common::status;
use common::status::{RawStatusPtr, ArrowError};
use buffer::{RawBufferPtr, Buf, Buffer};
use libc;
use std::ffi::CString;

#[repr(C)]
pub enum AccessMode {
  READ,
  WRITE,
  READWRITE
}

pub enum MemorySource {
  MEMORY_MAPPED_SRC { data: MemoryMappedSource }
}

pub struct MemoryMappedSource {
  raw_source: RawMemoryMappedSourceMutPtr
}

impl MemoryMappedSource {
  pub fn open(path: String, mode: AccessMode) -> MemoryMappedSource {
    MemoryMappedSource {
      raw_source: unsafe { open_mmap_src(string_to_cstr!(path), mode) }
    }
  }

  pub fn raw_source(&self) -> RawMemoryMappedSourceMutPtr {
    self.raw_source
  }

  pub fn close(&self) -> Result<&MemoryMappedSource, ArrowError> {
    let s = unsafe { close_mmap_src(self.raw_source) };
    result_from_status!(s, self)
  }

  pub fn read(&self, pos: i64, nbytes: i64) -> Buf {
    Buf::from_raw( unsafe { read_at_mmap_src(self.raw_source, pos, nbytes) } )
  }

  pub fn write(&self, pos: i64, data: *const u8, nbytes: i64) -> Result<&MemoryMappedSource, ArrowError> {
    let s = unsafe { write_mmap_src(self.raw_source, pos, data, nbytes) };
    result_from_status!(s, self)
  }

  pub fn size(&self) -> i64 {
    unsafe { mmap_src_size(self.raw_source) }
  }
}

impl Drop for MemoryMappedSource {
  fn drop(&mut self) {
    unsafe { release_mmap_src(self.raw_source) }
  }
}

pub enum RawMemoryMappedSource {}

pub type RawMemoryMappedSourceMutPtr = *mut RawMemoryMappedSource;

extern "C" {
  pub fn open_mmap_src(path: *const libc::c_char, mode: AccessMode) -> RawMemoryMappedSourceMutPtr;
  pub fn release_mmap_src(src: RawMemoryMappedSourceMutPtr);
  pub fn close_mmap_src(src: RawMemoryMappedSourceMutPtr) -> RawStatusPtr;
  pub fn read_at_mmap_src(src: RawMemoryMappedSourceMutPtr, pos: i64, nbytes: i64) -> RawBufferPtr;
  pub fn write_mmap_src(src: RawMemoryMappedSourceMutPtr, pos: i64, data: *const u8, nbytes: i64) -> RawStatusPtr;
  pub fn mmap_src_size(src: RawMemoryMappedSourceMutPtr) -> i64;
}