#![feature(cstr_from_bytes)]
#![feature(heap_api)]
#![feature(alloc)]

extern crate libc;
extern crate arrow;
extern crate alloc;

#[cfg(test)]
mod tests {
  use std::ffi::{CString, CStr};
  use std::ptr;
  use std::fs;
  use std::fs::File;
  use std::slice;
  use alloc::heap;

  use arrow::buffer;
  use arrow::column;
  use arrow::table;
  use arrow::ty;
  use arrow::array;
  use arrow::ipc::memory;
  use arrow::ipc::adapter;
  use arrow::types::primitive;
  use arrow::common::memory_pool;
  use arrow::common::status;

  #[test]
  fn test_raw_field() {
    unsafe {
      let dt = ty::new_primitive_type(ty::Ty::INT32);
      assert_eq!(4, ty::value_size(dt));
      assert_eq!(CStr::from_bytes_with_nul(b"int32\0").unwrap(),
        CStr::from_ptr(ty::data_type_to_string(dt)));

      let dt2 = ty::new_primitive_type(ty::Ty::INT32);
      assert!(ty::data_type_equals(dt, dt2));

      let fp = ty::new_field(CString::new("f0").unwrap().as_ptr(), dt, false);
      assert_eq!(CStr::from_bytes_with_nul(b"f0: int32 not null\0").unwrap(),
        CStr::from_ptr(ty::field_to_string(fp)));

      let fp2 = ty::new_field(CString::new("f0").unwrap().as_ptr(), dt2, false);
      assert!(ty::field_equals(fp, fp2));

      let fields = [fp, fp2];
      let struct_field = ty::new_struct_type(2, &fields);
      assert_eq!(CStr::from_bytes_with_nul(b"struct<f0: int32, f0: int32>\0").unwrap(),
        CStr::from_ptr(ty::data_type_to_string(struct_field)));

      ty::release_field(fp);
      ty::release_field(fp2);
      ty::release_data_type(dt);
      ty::release_data_type(dt2);
    }
  }

  #[test]
  fn test_field() {
    use arrow::ty::{DataTypeProvider, Field};

    let ty_provider = DataTypeProvider::new();

    let f1 = Field::new(String::from("f1"), ty_provider.i32(), false);
    let f2 = Field::new(String::from("f1"), ty_provider.i32(), false);

    assert_eq!(f1, f2);
    assert_eq!(String::from("f1: int32 not null"), f1.to_string());
  }

  #[test]
  fn test_raw_schema() {
    unsafe {
      let int_type = ty::new_primitive_type(ty::Ty::INT32);
      let float_type = ty::new_primitive_type(ty::Ty::FLOAT);
      let string_type = ty::new_string_type();

      let f0 = ty::new_field(CString::new("f0").unwrap().as_ptr(), int_type, false);
      let f1 = ty::new_field(CString::new("f1").unwrap().as_ptr(), float_type, false);
      let f2 = ty::new_field(CString::new("f2").unwrap().as_ptr(), string_type, false);
      let fields = [f0, f1, f2];


      let s = ty::new_schema(3, &fields);
      ty::release_schema(s);

      ty::release_field(f0);
      ty::release_field(f1);
      ty::release_field(f2);
      ty::release_data_type(string_type);
      ty::release_data_type(float_type);
      ty::release_data_type(int_type);
    }
  }

  #[test]
  fn test_schema() {
    use arrow::ty::{DataTypeProvider, Schema, Field};

    let ty_provider = DataTypeProvider::new();
    let f1 = Field::new(String::from("f1"), ty_provider.i32(), false);
    let f2 = Field::new(String::from("f2"), ty_provider.f32(), false);
    let f3 = Field::new(String::from("f3"), ty_provider.u64(), true);

    let schema = Schema::new(&[&f1, &f2, &f3]);
    assert_eq!(String::from("f1: int32 not null\nf2: float not null\nf3: uint64"), schema.to_string());

    let f1 = Field::new(String::from("f1"), ty_provider.i32(), false);
    let f2 = Field::new(String::from("f2"), ty_provider.f32(), false);
    let f3 = Field::new(String::from("f3"), ty_provider.u64(), true);
    let schema2 = Schema::new(&[&f1, &f2, &f3]);

    assert_eq!(schema, schema2);
  }

  #[test]
  fn test_raw_buffer() {

    unsafe {
      let pool = memory_pool::default_mem_pool();
      let buf_builder = buffer::new_buf_builder(pool);
      let val: u8 = 10;

      let s = buffer::raw_append_buf_builder(buf_builder, &val, 1);
      assert!(status::ok(s));
      status::release_status(s);

      let s = buffer::resize_buf_builder(buf_builder, 100);
      assert!(status::ok(s));
      status::release_status(s);

      assert_eq!(1, buffer::buf_builder_len(buf_builder));
      assert_eq!(128, buffer::buf_builder_capa(buf_builder));

      let buf = buffer::finish_buf_builder(buf_builder);
      assert_eq!(100, buffer::buf_size(buf));

      let s = buffer::resize_buf(buf, 50);
      assert!(status::ok(s));
      assert_eq!(50, buffer::buf_size(buf));
      assert_eq!(128, buffer::buf_capa(buf));

      buffer::release_buf(buf);
      buffer::release_buf_builder(buf_builder);
    }
  }

  #[test]
  fn test_buffer() {
    use arrow::common::memory_pool::MemoryPool;
    use arrow::buffer::{BufferBuilder, Buffer, Resizable, Mutable};

    let pool = MemoryPool::default();
    let mut builder = BufferBuilder::new(&pool);
    let val: u8 = 10;

    let len = match builder.raw_append(&val, 5) {
      Ok(len) => len,
      Err(e) => panic!("append failed: {}", e.message())
    };
    assert_eq!(5, len);

    builder.resize(200);
    assert_eq!(5, len);
    assert_eq!(256, builder.capacity());

    let mut buf = builder.finish();
    assert_eq!(200, buf.size());
    assert_eq!(256, buf.capacity());

    let buf = match buf.resize(50) {
      Ok(buf) => buf,
      Err(e) => panic!("resize failed: {}", e.message())
    };
    assert_eq!(50, buf.size());
    assert_eq!(256, buf.capacity());
  }

  #[test]
  fn test_raw_array() {

    unsafe {
      // FIXME: using the single memory pool makes difficult to verify the amount of allocated memory
      let pool = memory_pool::default_mem_pool();
      let mem_before = memory_pool::num_bytes_alloc(pool);

      let uint8 = ty::new_primitive_type(ty::Ty::UINT8);
      let builder = primitive::new_u8_arr_builder(pool, uint8);
      let values: Vec<u8> = (0..32).collect();

      let s = primitive::append_u8_arr_builder(builder, values.as_ptr(), 32, ptr::null());
      assert!(status::ok(s));
      status::release_status(s);

      let arr = primitive::finish_u8_arr_builder(builder);

      let u8_ty = ty::new_primitive_type(ty::Ty::UINT8);
      assert!(ty::data_type_equals(u8_ty, array::arr_type(arr)));
      ty::release_data_type(u8_ty);

      assert_eq!(32, array::arr_len(arr));

      for i in 0..32 {
        assert_eq!(i as u8, primitive::u8_arr_value(arr, i));
      }

      array::release_arr(arr);

      assert_eq!(mem_before, memory_pool::num_bytes_alloc(pool));
    }
  }

  #[test]
  fn test_array() {
    use arrow::common::memory_pool::MemoryPool;
    use arrow::ty::{DataTypeProvider, DataType};
    use arrow::types::primitive::{U8ArrayBuilder, PrimitiveArray};
    use arrow::array::Array;

    let ty_provider = DataTypeProvider::new();
    let pool = MemoryPool::default();

    let mut builder = U8ArrayBuilder::new(&pool, ty_provider.u8());
    let values: Vec<u8> = (0..32).collect();

    let mut builder = match builder.append(&values, ptr::null()) {
      Ok(builder) => builder,
      Err(e) => panic!("append failed: {}", e.message())
    };
    let array = builder.finish();
    assert_eq!(ty_provider.u8(), &array.data_type());
    assert_eq!(32, array.len());

    for i in 0..32 {
      assert_eq!(i as u8, array.value(i));
    }
  }

  #[test]
  fn test_raw_column() {

    unsafe {
      let pool = memory_pool::default_mem_pool();
      let f32_ty = ty::new_primitive_type(ty::Ty::FLOAT);
      let f1 = ty::new_field(CString::new("f1").unwrap().as_ptr(), f32_ty, false);
      let values: Vec<f32> = (0..32).map(|i| i as f32).collect();
      let builder = primitive::new_f32_arr_builder(pool, f32_ty);

      let s = primitive::append_f32_arr_builder(builder, values.as_ptr(), 32, ptr::null());
      assert!(status::ok(s));
      status::release_status(s);

      let arr = primitive::finish_f32_arr_builder(builder);
      assert_eq!(32, array::arr_len(arr));

      let col = column::new_column_from_arr(f1, arr);
      assert_eq!(32, column::column_len(col));
      assert_eq!(0, column::column_null_count(col));
      assert!(ty::data_type_equals(f32_ty, column::column_type(col)));
      let s = column::validate_column_data(col);
      assert!(status::ok(s));
      status::release_status(s);

      column::release_column(col);

      array::release_arr(arr);
      ty::release_field(f1);
      ty::release_data_type(f32_ty);
    }
  }

  #[test]
  fn test_column() {
    use arrow::common::memory_pool::MemoryPool;
    use arrow::ty::{DataTypeProvider, DataType, Field};
    use arrow::types::primitive::{F32ArrayBuilder, PrimitiveArray};
    use arrow::array::Array;
    use arrow::column::Column;

    let type_provider = DataTypeProvider::new();
    let pool = MemoryPool::default();
    let f1 = Field::new(String::from("f1"), type_provider.f32(), false);
    let values: Vec<f32> = (0..32).map(|i| i as f32).collect();
    let mut builder = F32ArrayBuilder::new(&pool, type_provider.f32());

    builder.append(&values, ptr::null());
    let array = builder.finish();
    assert_eq!(32, array.len());

    let col = Column::from_array(&f1, array.as_base());
    assert_eq!(32, col.len());
    assert_eq!(0, col.null_count());
    assert_eq!(type_provider.f32(), &col.data_type());
    let arrays = match col.validate_data() {
      Ok(arrays) => arrays,
      Err(e) => panic!("column data validation failed: {}", e.message())
    };
  }

  #[test]
  fn test_raw_row_batch() {
    unsafe {
      let pool = memory_pool::default_mem_pool();
      let f32_ty = ty::new_primitive_type(ty::Ty::FLOAT);
      let f1 = ty::new_field(CString::new("f1").unwrap().as_ptr(), f32_ty, false);
      let fields = [f1];
      let schema = ty::new_schema(1, &fields);
      let values: Vec<f32> = (0..32).map(|i| i as f32).collect();

      let builder = primitive::new_f32_arr_builder(pool, f32_ty);
      let s = primitive::append_f32_arr_builder(builder, values.as_ptr(), 32, ptr::null());
      status::release_status(s);
      let arrs = [primitive::finish_f32_arr_builder(builder)];

      let row_batch = table::new_row_batch(schema, 32, &arrs, 1);

      assert!(ty::schema_equals(schema, table::row_batch_schema(row_batch)));
      assert!(array::arr_equals(arrs[0], table::row_batch_column(row_batch, 0)));
      assert_eq!(32, table::row_batch_num_rows(row_batch));
      assert_eq!(1, table::row_batch_num_cols(row_batch));

      table::release_row_batch(row_batch);
      array::release_arr(arrs[0]);
      ty::release_schema(schema);
      ty::release_field(f1);
      ty::release_data_type(f32_ty);
    }
  }

  #[test]
  fn test_row_batch() {
    use arrow::common::memory_pool::MemoryPool;
    use arrow::ty::{DataTypeProvider, DataType, Field, Schema};
    use arrow::types::primitive::{F32ArrayBuilder, I32ArrayBuilder, PrimitiveArray};
    use arrow::array::Array;
    use arrow::table::RowBatch;

    let type_provider = DataTypeProvider::new();
    let pool = MemoryPool::default();
    let f1 = Field::new(String::from("f1"), type_provider.i32(), false);
    let f2 = Field::new(String::from("f2"), type_provider.f32(), true);
    let schema = Schema::new(&[&f1, &f2]);
    let values1: Vec<i32> = (0..100).map(|i| i as i32).collect();
    let values2: Vec<f32> = (0..100).map(|i| i as f32).collect();

    let mut builder1 = I32ArrayBuilder::new(&pool, type_provider.i32());
    let mut builder2 = F32ArrayBuilder::new(&pool, type_provider.f32());
    builder1.append(&values1, ptr::null());
    builder2.append(&values2, ptr::null());

    let arrays = [builder1.finish_as_base(), builder2.finish_as_base()];
    let row_batch = RowBatch::new(&schema, 100, &arrays);

    assert_eq!(schema, row_batch.schema());
    assert_eq!(&arrays[0], &row_batch.column(0));
    assert_eq!(&arrays[1], &row_batch.column(1));
    assert_eq!(100, row_batch.row_num());
    assert_eq!(2, row_batch.column_num());
  }

  #[test]
  fn test_raw_table() {
    unsafe {
      let pool = memory_pool::default_mem_pool();
      let f32_ty = ty::new_primitive_type(ty::Ty::FLOAT);
      let f1 = ty::new_field(CString::new("f1").unwrap().as_ptr(), f32_ty, false);
      let fields = [f1];
      let schema = ty::new_schema(1, &fields);
      let values: Vec<f32> = (0..32).map(|i| i as f32).collect();

      let builder = primitive::new_f32_arr_builder(pool, f32_ty);
      let s = primitive::append_f32_arr_builder(builder, values.as_ptr(), 32, ptr::null());
      status::release_status(s);
      let arrs = [primitive::finish_f32_arr_builder(builder)];
      let cols = [column::new_column_from_arr(f1, arrs[0])];

      let table = table::new_table(CString::new("t1").unwrap().as_ptr(), schema, &cols, 1);
      assert!(ty::schema_equals(schema, table::table_schema(table)));
      assert_eq!(1, table::table_num_cols(table));
      assert_eq!(32, table::table_num_rows(table));
      //      assert!(column::column_equals(cols[0], table::table_column(table, 0)));
      let s = table::validate_table_cols(table);
      assert!(status::ok(s));
      status::release_status(s);

      table::release_table(table);
      column::release_column(cols[0]);
      array::release_arr(arrs[0]);
      ty::release_schema(schema);
      ty::release_field(f1);
      ty::release_data_type(f32_ty);
    }
  }

  #[test]
  fn test_table() {
    use arrow::common::memory_pool::MemoryPool;
    use arrow::ty::{DataTypeProvider, DataType, Field, Schema};
    use arrow::types::primitive::{F32ArrayBuilder, I32ArrayBuilder, PrimitiveArray};
    use arrow::array::Array;
    use arrow::table::Table;
    use arrow::column::Column;

    let type_provider = DataTypeProvider::new();
    let pool = MemoryPool::default();
    let f1 = Field::new(String::from("f1"), type_provider.i32(), false);
    let f2 = Field::new(String::from("f2"), type_provider.f32(), true);
    let schema = Schema::new(&[&f1, &f2]);
    let values1: Vec<i32> = (0..100).map(|i| i as i32).collect();
    let values2: Vec<f32> = (0..100).map(|i| i as f32).collect();

    let mut builder1 = I32ArrayBuilder::new(&pool, type_provider.i32());
    let mut builder2 = F32ArrayBuilder::new(&pool, type_provider.f32());
    builder1.append(&values1, ptr::null());
    builder2.append(&values2, ptr::null());

    let arrays = [builder1.finish_as_base(), builder2.finish_as_base()];
    let cols = [Column::from_array(&f1, &arrays[0]), Column::from_array(&f2, &arrays[1])];
    let table = Table::new(String::from("t1"), &schema, &cols);

    assert_eq!(schema, table.schema());
    assert_eq!(2, table.column_num());
    assert_eq!(100, table.row_num());
    let table = match table.validate_columns() {
      Ok(table) => table,
      Err(e) => panic!("table validation failed: {}", e.message())
    };
  }

  #[test]
  fn test_raw_mem_src() {
    let file_name = "test_raw_mem_src.dat";
    let mut f = File::create(file_name).unwrap();
    f.set_len(32).unwrap();
    f.sync_all().unwrap();

    unsafe {
      let src = memory::open_mmap_src(CString::new(file_name).unwrap().as_ptr(),
                                      memory::AccessMode::READ_WRITE);
      let values: Vec<u8> = (0..32).collect();
      let origin = values.clone();
      let s = memory::write_mmap_src(src, 0, values.as_ptr(), 32);
      assert!(status::ok(s));
      status::release_status(s);

      let s = memory::close_mmap_src(src);
      assert!(status::ok(s));
      status::release_status(s);
      memory::release_mmap_src(src);

      let src = memory::open_mmap_src(CString::new(file_name).unwrap().as_ptr(),
                                      memory::AccessMode::READ_ONLY);
      let buf = memory::read_at_mmap_src(src, 0, 32);
      let v = slice::from_raw_parts(buffer::buf_data(buf), 32);
      assert_eq!(&origin, &v);
      buffer::release_buf(buf);

      let s = memory::close_mmap_src(src);
      assert!(status::ok(s));
      status::release_status(s);
      memory::release_mmap_src(src);
    }

    fs::remove_file(file_name).unwrap();
  }

  #[test]
  fn test_mem_src() {
    use arrow::buffer::Buffer;
    use arrow::ipc::memory::MemoryMappedSource;

    let file_name = "test_mem_src.dat";
    let mut f = File::create(file_name).unwrap();
    f.set_len(32).unwrap();
    f.sync_all().unwrap();

    let values: Vec<u8> = (0..32).collect();
    let origin = values.clone();

    let src = MemoryMappedSource::open(String::from(file_name), memory::AccessMode::READ_WRITE);
    let src = match src.write(0, values.as_ptr(), 32) {
      Ok(src) => src,
      Err(e) => panic!("write failed: {}", e.message())
    };
    let src = match src.close() {
      Ok(src) => src,
      Err(e) => panic!("close failed: {}", e.message())
    };

    let src = MemoryMappedSource::open(String::from(file_name), memory::AccessMode::READ_ONLY);
    let buf = src.read(0, 32);
    let s = unsafe { slice::from_raw_parts(buf.data(), 32) };
    assert_eq!(&origin, &s);

    let src = match src.close() {
      Ok(src) => src,
      Err(e) => panic!("close failed: {}", e.message())
    };

    fs::remove_file(file_name).unwrap();
  }

  #[test]
  fn test_raw_adapter() {
    let file_name = "test_raw_adapter.dat";
    unsafe {
      let pool = memory_pool::default_mem_pool();
      let f32_ty = ty::new_primitive_type(ty::Ty::FLOAT);
      let f1 = ty::new_field(CString::new("f1").unwrap().as_ptr(), f32_ty, false);
      let fields = [f1];
      let schema = ty::new_schema(1, &fields);
      let values: Vec<f32> = (0..32).map(|i| i as f32).collect();

      let builder = primitive::new_f32_arr_builder(pool, f32_ty);
      let s = primitive::append_f32_arr_builder(builder, values.as_ptr(), 32, ptr::null());
      status::release_status(s);
      let arrs = [primitive::finish_f32_arr_builder(builder)];

      let row_batch = table::new_row_batch(schema, 32, &arrs, 1);

      let batch_size = adapter::c_api::get_row_batch_size(row_batch);

      let mut f = File::create(file_name).unwrap();
      f.set_len(batch_size as u64).unwrap();
      f.sync_all().unwrap();

      let src = memory::open_mmap_src(CString::new(file_name).unwrap().as_ptr(),
                                      memory::AccessMode::READ_WRITE);
      let header_pos = adapter::c_api::write_row_batch(src, row_batch, 0, 64);

      let s = memory::close_mmap_src(src);
      assert!(status::ok(s));
      status::release_status(s);
      memory::release_mmap_src(src);
      table::release_row_batch(row_batch);

      let src = memory::open_mmap_src(CString::new(file_name).unwrap().as_ptr(),
                                      memory::AccessMode::READ_ONLY);

      let reader = adapter::c_api::open_row_batch_reader(src, header_pos);
      let row_batch = adapter::c_api::get_row_batch(reader, schema);

      let col = table::row_batch_column(row_batch, 0);
      assert!(array::arr_equals(arrs[0], col));

      let s = memory::close_mmap_src(src);
      assert!(status::ok(s));
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

  #[test]
  fn test_adapter() {
    use arrow::buffer::Buffer;
    use arrow::ipc::memory::MemoryMappedSource;
    use arrow::common::memory_pool::MemoryPool;
    use arrow::ty::{DataTypeProvider, DataType, Field, Schema};
    use arrow::types::primitive::{F32ArrayBuilder, I32ArrayBuilder, PrimitiveArray};
    use arrow::array::Array;
    use arrow::table::RowBatch;
    use arrow::ipc::adapter;
    use arrow::ipc::adapter::RowBatchReader;

    // prepare test
    let type_provider = DataTypeProvider::new();
    let pool = MemoryPool::default();
    let f1 = Field::new(String::from("f1"), type_provider.i32(), false);
    let f2 = Field::new(String::from("f2"), type_provider.f32(), true);
    let schema = Schema::new(&[&f1, &f2]);
    let values1: Vec<i32> = (0..100).map(|i| i as i32).collect();
    let values2: Vec<f32> = (0..100).map(|i| i as f32).collect();

    // create a row batch
    let mut builder1 = I32ArrayBuilder::new(&pool, type_provider.i32());
    let mut builder2 = F32ArrayBuilder::new(&pool, type_provider.f32());
    builder1.append(&values1, ptr::null());
    builder2.append(&values2, ptr::null());

    let arrays = [builder1.finish_as_base(), builder2.finish_as_base()];
    let row_batch = RowBatch::new(&schema, 100, &arrays);
    let batch_size = row_batch.size();

    let file_name = "test_adapter.dat";
    let mut f = File::create(file_name).unwrap();
    f.set_len(batch_size as u64).unwrap();
    f.sync_all().unwrap();

    // write row batch
    let src = MemoryMappedSource::open(String::from(file_name), memory::AccessMode::READ_WRITE);
    let header_pos = adapter::write_row_batch(&src, &row_batch, 0);

    let src = match src.close() {
      Ok(src) => src,
      Err(e) => panic!("close failed: {}", e.message())
    };

    // read row batch
    let src = MemoryMappedSource::open(String::from(file_name), memory::AccessMode::READ_ONLY);
    let batch_reader = RowBatchReader::open(&src, header_pos);
    let row_batch = batch_reader.read(&schema);
    let col = row_batch.column(0);
    assert_eq!(arrays[0], col);

    let src = match src.close() {
      Ok(src) => src,
      Err(e) => panic!("close failed: {}", e.message())
    };

    fs::remove_file(file_name).unwrap();
  }
}
