#[macro_use]
pub mod status;
pub mod memory_pool;

#[macro_export]
macro_rules! cstr_to_string {
  ($str:expr) => (
    String::from_utf8(Vec::from(CStr::from_ptr($str).to_bytes())).unwrap()
  );
}

#[macro_export]
macro_rules! string_to_cstr {
  ($str:expr) => (
    CString::new($str).unwrap().into_raw()
  );
}

#[cfg(test)]
mod tests {

  #[test]
  fn test_mem_pool() {
    use common::memory_pool;
    use std::ptr;
    use libc;
    use common::status;
    use std::mem;

    unsafe {
      let pool = memory_pool::default_mem_pool();
      let buffer: *mut u8 = ptr::null_mut();

      let init_mem_bytes = memory_pool::num_bytes_alloc(pool);

      let status = memory_pool::mem_alloc(pool, buffer, 64);
      assert!(status::ok(status));
      status::release_status(status);
      assert_eq!(init_mem_bytes + 64, memory_pool::num_bytes_alloc(pool));

      memory_pool::mem_free(pool, buffer, 32);
      assert_eq!(init_mem_bytes + 32, memory_pool::num_bytes_alloc(pool));

      memory_pool::mem_free(pool, buffer, 32);
      assert_eq!(init_mem_bytes, memory_pool::num_bytes_alloc(pool));
    }
  }
}