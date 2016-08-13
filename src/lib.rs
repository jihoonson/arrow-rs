#![feature(test)]
extern crate libc;
extern crate test;

pub mod common;
pub mod types;
pub mod ty;
pub mod array;
pub mod buffer;
pub mod column;
pub mod table;
pub mod ipc;

mod benchmarks {
  use std::ffi::{CString, CStr};
  use std::ptr;
  use std::fs;
  use std::fs::File;
  use std::slice;
  use test::Bencher;

  use table;
  use ty;
  use array;
  use ipc::memory;
  use ipc::adapter;
  use types::primitive;
  use common::memory_pool;
  use common::status;

  #[bench]
  fn bench_adapter(b: &mut Bencher) {
    unsafe {
      let pool = memory_pool::default_mem_pool();
      let f32_ty = ty::new_primitive_type(ty::Ty::FLOAT);
      let f1 = ty::new_field(CString::new("f1").unwrap().as_ptr(), f32_ty, false);
      let fields = [f1];
      let schema = ty::new_schema(1, &fields);
      let values: Vec<f32> = (0..1000000).map(|i| i as f32).collect();
      let val_len = values.len() as i32;

      let builder = primitive::new_f32_arr_builder(pool, f32_ty);
      let s = primitive::append_f32_arr_builder(builder, values.as_ptr(), val_len, ptr::null());
      status::release_status(s);
      let arrs = [primitive::finish_f32_arr_builder(builder)];

      let row_batch = table::new_row_batch(schema, val_len, &arrs, 1);

      let batch_size = adapter::get_row_batch_size(row_batch);

      let mut f = File::create("bench_adapter.dat").unwrap();
      f.set_len(batch_size as u64).unwrap();
      f.sync_all().unwrap();

      let src = memory::open_mmap_src(CString::new("bench_adapter.dat").unwrap().as_ptr(),
                                      memory::AccessMode::READ_WRITE);
      let header_pos = adapter::write_row_batch(src, row_batch, 0, 64);

      let s = memory::close_mmap_src(src);
      status::release_status(s);
      memory::release_mmap_src(src);
      table::release_row_batch(row_batch);

      let src = memory::open_mmap_src(CString::new("bench_adapter.dat").unwrap().as_ptr(),
                                      memory::AccessMode::READ_ONLY);

      let reader = adapter::open_row_batch_reader(src, header_pos);
      let row_batch = adapter::get_row_batch(reader, schema);

      let col = table::row_batch_column(row_batch, 0);

      b.iter(|| {
//        let mut result : Vec<f32> = Vec::new();
//
//        for i in 0..val_len {
//          let f = primitive::f32_arr_value(col, i);
//          if f >= 10000. && f < 100000. {
//            result.push(f);
//          }
//        }

        let result = (0..val_len).filter(|i| {
          let f = primitive::f32_arr_value(col, *i);
          f >= 10000. && f < 100000.
        }).map(|i| primitive::f32_arr_value(col, i)).collect::<Vec<f32>>();
      });

      let s = memory::close_mmap_src(src);
      status::release_status(s);
      memory::release_mmap_src(src);

      adapter::release_row_batch_reader(reader);
      table::release_row_batch(row_batch);

      array::release_arr(arrs[0]);
      ty::release_schema(schema);
      ty::release_field(f1);
      ty::release_data_type(f32_ty);
    }

    fs::remove_file("bench_adapter.dat").unwrap();
  }
}