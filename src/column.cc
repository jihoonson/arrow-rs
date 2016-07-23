#include "column.h"

ChunkedArrayBox* new_chunked_arr(ArrayBox* arrays[], int arr_len) {
  std::vector<std::shared_ptr<Array>> arr_vec (arr_len);
  for (int i = 0; i < arr_len; i++) {
    arr_vec.push_back(arrays[i]->sp);
  }

  ChunkedArrayBox* chunked_arr = new ChunkedArrayBox;
  chunked_arr->sp = std::make_shared<ChunkedArray>(arr_vec);
  chunked_arr->p = chunked_arr->sp.get();
  return chunked_arr;
}

void release_chunked_arr(ChunkedArrayBox* chunked_arr) {
  if (chunked_arr) {
    delete chunked_arr;
  }
}

ColumnBox* new_column_from_arr(FieldBox* field, ArrayBox* arr) {
  ColumnBox* column = new ColumnBox;
  column->sp = std::make_shared<Column>(field->sp, arr->sp);
  column->p = column->sp.get();
  return column;
}

ColumnBox* new_column_from_chunked_arr(FieldBox* field, ChunkedArrayBox* arr) {
  ColumnBox* column = new ColumnBox;
      column->sp = std::make_shared<Column>(field->sp, arr->sp);
      column->p = column->sp.get();
      return column;
}

void release_column(ColumnBox* column) {
  if (column) {
    delete column;
  }
}

int64_t column_len(ColumnBox* column) {
  return column->p->length();
}

int64_t column_null_count(ColumnBox* column) {
  return column->p->null_count();
}

DataTypeBox* column_type(ColumnBox* column) {
  DataTypeBox* data_type = new DataTypeBox;
  data_type->sp = column->p->type();
  data_type->p = data_type->sp.get();
  return data_type;
}

ChunkedArrayBox* column_data(ColumnBox* column) {
  ChunkedArrayBox* chunked_arr = new ChunkedArrayBox;
  chunked_arr->sp = column->p->data();
  chunked_arr->p = chunked_arr->sp.get();
  return chunked_arr;
}

StatusBox* validate_column_data(ColumnBox* column) {
  StatusBox* status = new StatusBox;
  status->status = column->p->ValidateData();
  return status;
}