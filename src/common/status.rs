use libc;

// Status code of arrow
// See arrow::StatusCode
pub enum StatusCode {
    OK = 0,
    OutOfMemory = 1,
    KeyError = 2,
    Invalid = 3,
    IOError = 4,

    NotImplemented = 10,
}

pub enum Status {}

// TODO: add safe APIs

extern "C" {
  pub fn new_status() -> *mut Status;
  pub fn release_status(status: *const Status);

  pub fn ok(status: *const Status) -> bool;
  pub fn is_oom(status: *const Status) -> bool;
  pub fn is_key_error(status: *const Status) -> bool;
  pub fn is_invalid(status: *const Status) -> bool;
  pub fn is_io_error(status: *const Status) -> bool;
  pub fn is_not_implemented(status: *const Status) -> bool;
  pub fn status_to_str(status: *const Status) -> *const libc::c_char;
  pub fn code_to_str(status: *const Status) -> *const libc::c_char;
  pub fn posix_code(status: *const Status) -> i16;
}