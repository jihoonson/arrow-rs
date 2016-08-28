use libc;
use std::ops::Drop;
use std::string::String;
use std::ffi::CStr;

#[macro_export]
macro_rules! result_from_status {
  ($s:ident, $result:expr) => (
    if unsafe { status::ok($s) } {
      unsafe { status::release_status($s) };
      Ok($result)
    } else {
      Err(ArrowError::new($s))
    }
  );
}

// Status code of arrow
// See arrow::StatusCode
#[repr(C)]
pub enum StatusCode {
    OK = 0,
    OutOfMemory = 1,
    KeyError = 2,
    Invalid = 3,
    IOError = 4,

    NotImplemented = 10,
}

pub struct ArrowError {
  code: StatusCode,
//  posix_code: i16,
  message: String
}

impl ArrowError {
  pub fn new(status: RawStatusPtr) -> ArrowError {
    unsafe {
      let code = code(status);
//      let posix_code = posix_code(status);
      let bytes = CStr::from_ptr(message(status)).to_bytes();
      let msg = String::from_utf8(Vec::from(bytes)).unwrap();
      release_status(status); // TODO: consider more reliable way to handle the raw status pointer

      ArrowError {
        code: code,
//        posix_code: posix_code,
        message: msg
      }
    }
  }

  pub fn code(&self) -> &StatusCode {
    &self.code
  }

  pub fn message(&self) -> &String {
    &self.message
  }
}

pub enum RawStatus {}

pub type RawStatusPtr = *const RawStatus;

extern "C" {
//  pub fn new_status() -> *mut Status;
  pub fn release_status(status: *const RawStatus);

  pub fn ok(status: *const RawStatus) -> bool;
  pub fn is_oom(status: *const RawStatus) -> bool;
  pub fn is_key_error(status: *const RawStatus) -> bool;
  pub fn is_invalid(status: *const RawStatus) -> bool;
  pub fn is_io_error(status: *const RawStatus) -> bool;
  pub fn is_not_implemented(status: *const RawStatus) -> bool;
  pub fn status_to_str(status: *const RawStatus) -> *const libc::c_char;
  pub fn code_to_str(status: *const RawStatus) -> *const libc::c_char;
//  pub fn posix_code(status: *const RawStatus) -> i16;
  pub fn code(status: *const RawStatus) -> StatusCode;
  pub fn message(status: RawStatusPtr) -> *const libc::c_char;
}