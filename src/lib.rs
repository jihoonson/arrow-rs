#![feature(cstr_from_bytes)]
extern crate libc;

mod common;
mod types;

#[cfg(test)]
mod tests {
  use types;
  use std::ffi::{CString, CStr};

  #[test]
  fn test_field() {
    unsafe {
      let dt = types::new_data_type(types::Ty::INT32);
      assert_eq!(4, types::value_size(dt));

      let fp = types::new_field(CString::new("f0").unwrap().as_ptr(), dt, false);
      assert_eq!(CStr::from_bytes_with_nul(b"f0: int32 not null\0").unwrap(),
        CStr::from_ptr(types::field_to_string(fp)));

      types::release_field(fp);
      types::release_data_type(dt);
    }
  }
}
