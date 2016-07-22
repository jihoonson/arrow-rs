#include "array.h"

bool arr_is_null(ArrayBox* arr, int i) {
  return arr->p->IsNull(i);
}

int32_t arr_len(ArrayBox* arr) {
  return arr->p->length();
}

int32_t arr_null_count(ArrayBox* arr) {
  return arr->p->null_count();
}

DataTypeBox* arr_type(ArrayBox* arr) {
  return arr->type;
}

Type::type arr_type_enum(ArrayBox* arr) {
  return arr->p->type_enum();
}

// TODO: null_bitmap()

// TODO: null_bitmap_data()

bool arr_equals_exact(ArrayBox* arr1, ArrayBox* arr2) {
  return arr1->p->EqualsExact(*(arr2->p));
}

bool arr_equals(ArrayBox* arr1, ArrayBox* arr2) {
  return arr1->p->Equals(arr2->sp);
}

bool arr_range_equals(ArrayBox* arr1, ArrayBox* arr2, int32_t start, int32_t end, int32_t other_start) {
  return arr1->p->RangeEquals(start, end, other_start, arr2->sp);
}

ArrayBox* new_null_arr(DataTypeBox* type, int32_t length) {
  ArrayBox* arr = new ArrayBox;
  arr->sp = std::make_shared<NullArray>(type->sp, length);
  arr->p = arr->sp.get();
  arr->type = type;
  return arr;
}

void release_arr(ArrayBox* arr) {
  if (arr) {
    delete arr->type;
    delete arr;
  }
}