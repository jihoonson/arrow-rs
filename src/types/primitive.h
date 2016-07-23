#ifndef PRIMITIVE_H
#define PRIMITIVE_H

#include "arrow/types/primitive.h"
#include "../array.h"
#include "../buffer.h"
#include "../ty.h"
#include "../common/memory_pool.h"
#include "../common/status.h"

using namespace arrow;

#define RAW_DATA_FUNC_DECL(NAME, TYPE, RS_TYPE)       \
  const TYPE* RS_TYPE##_arr_raw_data(ArrayBox* arr) {        \
    return ((NAME*)arr->p)->raw_data();               \
  }

#define VALUE_FUNC_DECL(NAME, TYPE, RS_TYPE)          \
  TYPE RS_TYPE##_arr_value(ArrayBox* arr, int i) {      \
    return ((NAME*)arr->p)->Value(i);                 \
  }

#define NEW_ARRAY_BUILDER_DECL(NAME, RS_TYPE)       \
  NAME* new_##RS_TYPE##_arr_builder(MemoryPool* pool, DataTypeBox* type) {   \
    return new NAME(pool, type->sp);                          \
  }

#define INIT_ARRAY_BUILDER_DECL(NAME, RS_TYPE)      \
  StatusBox* init_##RS_TYPE##_arr_builder(NAME* builder, int32_t capacity) {  \
    StatusBox* box = new StatusBox; \
    box->status = builder->Init(capacity);  \
    return box; \
  }

#define APPEND_ARRAY_BUILDER_DECL(NAME, TYPE, RS_TYPE)    \
  StatusBox* append_##RS_TYPE##_arr_builder(NAME* builder, const TYPE* values, int32_t length, const uint8_t* valid_bytes) {  \
    StatusBox* box = new StatusBox; \
    box->status = builder->Append(values, length, valid_bytes); \
    return box; \
  }

#define FINISH_ARRAY_BUILDER_DECL(NAME, RS_TYPE)  \
  ArrayBox* finish_##RS_TYPE##_arr_builder(NAME* builder) { \
    ArrayBox* arr = new ArrayBox; \
    arr->sp = builder->Finish();  \
    arr->p = arr->sp.get(); \
    arr->type = new DataTypeBox;  \
    arr->type->sp = arr->p->type(); \
    arr->type->p = arr->type->sp.get(); \
    return arr; \
  }

extern "C" {

  // Primitive array

  BufferBox* arr_data(ArrayBox* arr) {
    BufferBox* buf = new BufferBox;
    buf->sp = reinterpret_cast<PrimitiveArray*>(arr->p)->data();
    buf->p = buf->sp.get();
    return buf;
  }

  RAW_DATA_FUNC_DECL(UInt8Array, uint8_t, u8);
  RAW_DATA_FUNC_DECL(Int8Array, int8_t, i8);
  RAW_DATA_FUNC_DECL(UInt16Array, uint16_t, u16);
  RAW_DATA_FUNC_DECL(Int16Array, int16_t, i16);
  RAW_DATA_FUNC_DECL(UInt32Array, uint32_t, u32);
  RAW_DATA_FUNC_DECL(Int32Array, int32_t, i32);
  RAW_DATA_FUNC_DECL(UInt64Array, uint64_t, u64);
  RAW_DATA_FUNC_DECL(Int64Array, int64_t, i64);
  RAW_DATA_FUNC_DECL(FloatArray, float, f32);
  RAW_DATA_FUNC_DECL(DoubleArray, double, f64);

  VALUE_FUNC_DECL(UInt8Array, uint8_t, u8);
  VALUE_FUNC_DECL(Int8Array, int8_t, i8);
  VALUE_FUNC_DECL(UInt16Array, uint16_t, u16);
  VALUE_FUNC_DECL(Int16Array, int16_t, i16);
  VALUE_FUNC_DECL(UInt32Array, uint32_t, u32);
  VALUE_FUNC_DECL(Int32Array, int32_t, i32);
  VALUE_FUNC_DECL(UInt64Array, uint64_t, u64);
  VALUE_FUNC_DECL(Int64Array, int64_t, i64);
  VALUE_FUNC_DECL(FloatArray, float, f32);
  VALUE_FUNC_DECL(DoubleArray, double, f64);

  // Primitive array builder

  NEW_ARRAY_BUILDER_DECL(UInt8Builder, u8);
  NEW_ARRAY_BUILDER_DECL(Int8Builder, i8);
  NEW_ARRAY_BUILDER_DECL(UInt16Builder, u16);
  NEW_ARRAY_BUILDER_DECL(Int16Builder, i16);
  NEW_ARRAY_BUILDER_DECL(UInt32Builder, u32);
  NEW_ARRAY_BUILDER_DECL(Int32Builder, i32);
  NEW_ARRAY_BUILDER_DECL(UInt64Builder, u64);
  NEW_ARRAY_BUILDER_DECL(Int64Builder, i64);
  NEW_ARRAY_BUILDER_DECL(FloatBuilder, f32);
  NEW_ARRAY_BUILDER_DECL(DoubleBuilder, f64);

  INIT_ARRAY_BUILDER_DECL(UInt8Builder, u8);
  INIT_ARRAY_BUILDER_DECL(Int8Builder, i8);
  INIT_ARRAY_BUILDER_DECL(UInt16Builder, u16);
  INIT_ARRAY_BUILDER_DECL(Int16Builder, i16);
  INIT_ARRAY_BUILDER_DECL(UInt32Builder, u32);
  INIT_ARRAY_BUILDER_DECL(Int32Builder, i32);
  INIT_ARRAY_BUILDER_DECL(UInt64Builder, u64);
  INIT_ARRAY_BUILDER_DECL(Int64Builder, i64);
  INIT_ARRAY_BUILDER_DECL(FloatBuilder, f32);
  INIT_ARRAY_BUILDER_DECL(DoubleBuilder, f64);

  // TODO: add reserve

  APPEND_ARRAY_BUILDER_DECL(UInt8Builder, uint8_t, u8);
  APPEND_ARRAY_BUILDER_DECL(Int8Builder, int8_t, i8);
  APPEND_ARRAY_BUILDER_DECL(UInt16Builder, uint16_t, u16);
  APPEND_ARRAY_BUILDER_DECL(Int16Builder, int16_t, i16);
  APPEND_ARRAY_BUILDER_DECL(UInt32Builder, uint32_t, u32);
  APPEND_ARRAY_BUILDER_DECL(Int32Builder, int32_t, i32);
  APPEND_ARRAY_BUILDER_DECL(UInt64Builder, uint64_t, u64);
  APPEND_ARRAY_BUILDER_DECL(Int64Builder, int64_t, i64);
  APPEND_ARRAY_BUILDER_DECL(FloatBuilder, float, f32);
  APPEND_ARRAY_BUILDER_DECL(DoubleBuilder, double, f64);

  FINISH_ARRAY_BUILDER_DECL(UInt8Builder, u8);
  FINISH_ARRAY_BUILDER_DECL(Int8Builder, i8);
  FINISH_ARRAY_BUILDER_DECL(UInt16Builder, u16);
  FINISH_ARRAY_BUILDER_DECL(Int16Builder, i16);
  FINISH_ARRAY_BUILDER_DECL(UInt32Builder, u32);
  FINISH_ARRAY_BUILDER_DECL(Int32Builder, i32);
  FINISH_ARRAY_BUILDER_DECL(UInt64Builder, u64);
  FINISH_ARRAY_BUILDER_DECL(Int64Builder, i64);
  FINISH_ARRAY_BUILDER_DECL(FloatBuilder, f32);
  FINISH_ARRAY_BUILDER_DECL(DoubleBuilder, f64);
}

#endif