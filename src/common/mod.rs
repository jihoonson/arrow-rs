#[macro_use]
pub mod status;
pub mod memory_pool;

#[macro_export]
macro_rules! cstr_to_string {
  ($str:expr) => (
    String::from_utf8(Vec::from( unsafe { CStr::from_ptr($str).to_bytes() } )).unwrap()
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

//  #[test]
//  fn test_raw_mem_pool() {
//    use common::memory_pool;
//    use std::ptr;
//    use libc;
//    use common::status;
//    use std::mem;
//
//    unsafe {
//      let pool = memory_pool::default_mem_pool();
//      let buffer: *mut u8 = ptr::null_mut();
//
//      let init_mem_bytes = memory_pool::num_bytes_alloc(pool);
//
//      let status = memory_pool::mem_alloc(pool, buffer, 64);
//      assert!(status::ok(status));
//      status::release_status(status);
//      // FIXME: using the single memory pool makes difficult to verify the amount of allocated memory
////      assert_eq!(init_mem_bytes + 64, memory_pool::num_bytes_alloc(pool));
//
//      memory_pool::mem_free(pool, buffer, 64);
//      // FIXME: using the single memory pool makes difficult to verify the amount of allocated memory
////      assert_eq!(init_mem_bytes, memory_pool::num_bytes_alloc(pool));
//    }
//  }

  #[test]
  fn test_mem_pool() {
    use common::memory_pool::MemoryPool;

    let mut pool = MemoryPool::default();
    let init_len = pool.len();

    let buf = match pool.alloc(64) {
      Ok(buf) => buf,
      Err(e) => panic!("allocation failed: {}", e.message())
    };
    // FIXME: using the single memory pool makes difficult to verify the amount of allocated memory
//    assert_eq!(init_len + 64, pool.len());

    pool.free(buf, 64);
    // FIXME: using the single memory pool makes difficult to verify the amount of allocated memory
//    assert_eq!(init_len, pool.len());
  }
}