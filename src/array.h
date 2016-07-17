#ifndef ARRAY_H
#define ARRAY_H

#include "arrow/array.h"
#include "ty.h"

using namespace arrow;

struct ArrayBox {
  std::shared_ptr<Array> sp;
  Array* array;
  DataTypeBox* type_box;
};

extern "C" {
  bool arr_is_null(ArrayBox* box, int i) {
    return box->array->IsNull(i);
  }

  int32_t arr_len(ArrayBox* box) {
    return box->array->length();
  }

  int32_t arr_null_count(ArrayBox* box) {
    return box->array->null_count();
  }

  DataTypeBox* arr_type(ArrayBox* box) {
    return box->type_box;
  }

  Type::type arr_type_enum(ArrayBox* box) {
    return box->array->type_enum();
  }

  // TODO: null_bitmap()

  // TODO: null_bitmap_data()

  bool arr_equals_exact(ArrayBox* box1, ArrayBox* box2) {
    return box1->array->EqualsExact(*(box2->array));
  }

  bool arr_equals(ArrayBox* box1, ArrayBox* box2) {
    return box1->array->Equals(box2->sp);
  }

  bool arr_range_equals(ArrayBox* box1, ArrayBox* box2, int32_t start, int32_t end, int32_t other_start) {
    return box1->array->RangeEquals(start, end, other_start, box2->sp);
  }

  ArrayBox* new_null_arr(DataTypeBox* type, int32_t length) {
    ArrayBox* box = new ArrayBox;
    box->sp = std::make_shared<NullArray>(type->sp, length);
    box->array = box->sp.get();
    box->type_box = type;
    return box;
  }
}

#endif