#![feature(concat_idents)]
#![feature(type_macros)]
#![feature(test)]
extern crate libc;
extern crate test;

#[macro_use]
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
  use std::mem;

  use table;
  use ty;
  use array;
  use ipc::memory;
  use ipc::adapter;
  use types::primitive;
  use common::memory_pool;
  use common::status;

  #[bench]
  fn bench_raw_adapter(b: &mut Bencher) {
    let file_name = "bench_raw_adapter.dat";
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

      let batch_size = adapter::c_api::get_row_batch_size(row_batch);

      let mut f = File::create(file_name).unwrap();
      f.set_len(batch_size as u64).unwrap();
      f.sync_all().unwrap();

      let src = memory::open_mmap_src(CString::new(file_name).unwrap().as_ptr(),
                                      memory::AccessMode::READ_WRITE);
      let header_pos = adapter::c_api::write_row_batch(src, row_batch, 0, 64);

      let s = memory::close_mmap_src(src);
      status::release_status(s);
      memory::release_mmap_src(src);
      table::release_row_batch(row_batch);

      let src = memory::open_mmap_src(CString::new(file_name).unwrap().as_ptr(),
                                      memory::AccessMode::READ_ONLY);

      let result = adapter::c_api::open_row_batch_reader(src, header_pos);
      assert!(status::ok((*result).status()));
      status::release_status((*result).status());

      let reader: adapter::c_api::RawRowBatchReaderPtr = mem::transmute((*result).result());
      adapter::c_api::release_arrow_result(result);

      let row_batch = adapter::c_api::get_row_batch(reader, schema);

      let col = table::row_batch_column(row_batch, 0);

      b.iter(|| {
        let result = (0..val_len).filter(|i| {
          let f = primitive::f32_arr_value(col, *i);
          f >= 10000. && f < 100000.
        }).map(|i| primitive::f32_arr_value(col, i)).collect::<Vec<f32>>();
      });

      let s = memory::close_mmap_src(src);
      status::release_status(s);
      memory::release_mmap_src(src);

      adapter::c_api::release_row_batch_reader(reader);
      table::release_row_batch(row_batch);

      array::release_arr(arrs[0]);
      ty::release_schema(schema);
      ty::release_field(f1);
      ty::release_data_type(f32_ty);
    }

    fs::remove_file(file_name).unwrap();
  }

  #[bench]
  fn bench_adapter(b: &mut Bencher) {
    use common::memory_pool::MemoryPool;
    use ty::{DataTypeProvider, Schema, Field};
    use types::primitive::{I32Array, F32Array, I32ArrayBuilder, F32ArrayBuilder, PrimitiveArray};
    use table::RowBatch;
    use ipc::memory::MemoryMappedSource;
    use array::Array;

    let type_provider = DataTypeProvider::new();
    let pool = MemoryPool::default();
    let f1 = Field::new(String::from("key"), type_provider.i32(), false);
    let f2 = Field::new(String::from("payload"), type_provider.f32(), false);
    let schema = Schema::new(&[&f1, &f2]);
    let values1: Vec<i32> = (0..1000000).map(|i| i as i32).collect();
    let values2: Vec<f32> = (0..1000000).map(|i| i as f32).collect();
    let val_len = values1.len() as i32;

    let mut builder1 = I32ArrayBuilder::new(&pool, type_provider.i32());
    let mut builder2 = F32ArrayBuilder::new(&pool, type_provider.f32());
    builder1.append(&values1, ptr::null());
    builder2.append(&values2, ptr::null());

    let arrays = [builder1.finish_as_base(), builder2.finish_as_base()];
    let row_batch = RowBatch::new(&schema, val_len, &arrays);
    let batch_size = row_batch.size();

    let file_name = "bench_adapter.dat";
    let mut f = File::create(file_name).unwrap();
    f.set_len(batch_size as u64).unwrap();
    f.sync_all().unwrap();

    let src = MemoryMappedSource::open(String::from(file_name), memory::AccessMode::READ_WRITE);
    let header_pos = adapter::write_row_batch(&src, &row_batch, 0);
    src.close();

    let src = MemoryMappedSource::open(String::from(file_name), memory::AccessMode::READ_ONLY);
    let reader = match adapter::RowBatchReader::open(&src, header_pos) {
      Ok(reader) => reader,
      Err(e) => panic!("Failed to open RowBatchReader: {}", e.message())
    };
    let row_batch = reader.read(&schema);
    let key_col: I32Array = row_batch.column(0);
    let payload_col: F32Array = row_batch.column(1);

    b.iter(|| {
      let result = (0..val_len).filter(|i| {
        let key = key_col.value(*i);
        key >= 10000 && key < 100000
      }).map(|i| payload_col.value(i)).collect::<Vec<f32>>();
    });

    src.close();

    fs::remove_file(file_name).unwrap();
  }
}