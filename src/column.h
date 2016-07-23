#ifndef COLUMN_H
#define COLUMN_H

#include "arrow/column.h"
#include "ty.h"
#include "array.h"
#include "common/status.h"

using namespace arrow;

struct ChunkedArrayBox {
  std::shared_ptr<ChunkedArray> sp;
  ChunkedArray* p;
};

struct ColumnBox {
  std::shared_ptr<Column> sp;
  Column* p;
};

extern "C" {
  ChunkedArrayBox* new_chunked_arr(ArrayBox* arrays[], int arr_len);

  void release_chunked_arr(ChunkedArrayBox* chunked_arr);

  ColumnBox* new_column_from_arr(FieldBox* field, ArrayBox* arr);

  ColumnBox* new_column_from_chunked_arr(FieldBox* field, ChunkedArrayBox* arr);

  void release_column(ColumnBox* column);

  int64_t column_len(ColumnBox* column);

  int64_t column_null_count(ColumnBox* column);

  DataTypeBox* column_type(ColumnBox* column);

  ChunkedArrayBox* column_data(ColumnBox* column);

  StatusBox* validate_column_data(ColumnBox* column);
}

#endif