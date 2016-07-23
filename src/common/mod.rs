pub mod status;
pub mod memory_pool;

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
      let mut status = status::new_status();

      let init_mem_bytes = memory_pool::num_bytes_alloc(pool);

      let alloc_result = memory_pool::mem_alloc(pool, buffer, 64, status);
      assert!(alloc_result);
      assert_eq!(init_mem_bytes + 64, memory_pool::num_bytes_alloc(pool));
      assert!(status::ok(status));
      status::release_status(status);

      memory_pool::mem_free(pool, buffer, 32);
      assert_eq!(init_mem_bytes + 32, memory_pool::num_bytes_alloc(pool));

      memory_pool::mem_free(pool, buffer, 32);
      assert_eq!(init_mem_bytes, memory_pool::num_bytes_alloc(pool));
    }
  }
}