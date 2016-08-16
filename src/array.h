#ifndef ARRAY_H
#define ARRAY_H

#include "arrow/array.h"
#include "arrow/types/primitive.h"
#include "ty.h"
#include "buffer.h"

using namespace arrow;

struct ArrayBox {
  std::shared_ptr<Array> sp;
  Array* p;
};

extern "C" {
  bool arr_is_null(ArrayBox* arr, int i);

  int32_t arr_len(ArrayBox* arr);

  int32_t arr_null_count(ArrayBox* arr);

  DataTypeBox* arr_type(ArrayBox* arr);

  Type::type arr_type_enum(ArrayBox* arr);

  // TODO: null_bitmap()

  // TODO: null_bitmap_data()

  bool arr_equals_exact(ArrayBox* arr1, ArrayBox* arr2);

  bool arr_equals(ArrayBox* arr1, ArrayBox* arr2);

  bool arr_range_equals(ArrayBox* arr1, ArrayBox* arr2, int32_t start, int32_t end, int32_t other_start);

  ArrayBox* new_null_arr(DataTypeBox* type, int32_t length);

  void release_arr(ArrayBox* arr);
}

#endif