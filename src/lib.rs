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
      let dt = types::new_primitive_type(types::Ty::INT32);
      assert_eq!(4, types::value_size(dt));
      assert_eq!(CStr::from_bytes_with_nul(b"int32\0").unwrap(),
        CStr::from_ptr(types::data_type_to_string(dt)));

      let dt2 = types::new_primitive_type(types::Ty::INT32);
      assert!(types::data_type_equals(dt, dt2));

      let fp = types::new_field(CString::new("f0").unwrap().as_ptr(), dt, false);
      assert_eq!(CStr::from_bytes_with_nul(b"f0: int32 not null\0").unwrap(),
        CStr::from_ptr(types::field_to_string(fp)));

      let fp2 = types::new_field(CString::new("f0").unwrap().as_ptr(), dt2, false);
      assert!(types::field_equals(fp, fp2));

      let fields = [fp, fp2];
      let struct_field = types::new_struct_type(2, &fields);
      assert_eq!(CStr::from_bytes_with_nul(b"struct<f0: int32, f0: int32>\0").unwrap(),
        CStr::from_ptr(types::data_type_to_string(struct_field)));

      types::release_field(fp);
      types::release_field(fp2);
      types::release_data_type(dt);
      types::release_data_type(dt2);
    }
  }
  
  #[test]
  fn test_schema() {
    unsafe {
      let int_type = types::new_primitive_type(types::Ty::INT32);
      let float_type = types::new_primitive_type(types::Ty::FLOAT);
      let string_type = types::new_string_type();

      let f0 = types::new_field(CString::new("f0").unwrap().as_ptr(), int_type, false);
      let f1 = types::new_field(CString::new("f1").unwrap().as_ptr(), float_type, false);
      let f2 = types::new_field(CString::new("f2").unwrap().as_ptr(), string_type, false);
      let fields = [f0, f1, f2];


      let s = types::new_schema(3, &fields);
      types::release_schema(s);

      types::release_field(f0);
      types::release_field(f1);
      types::release_field(f2);
      types::release_data_type(string_type);
      types::release_data_type(float_type);
      types::release_data_type(int_type);
    }
  }
}
