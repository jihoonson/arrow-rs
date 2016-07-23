#ifndef TABLE_H
#define TABLE_H

#include "ty.h"
#include "column.h"
#include "array.h"
#include "common/status.h"
#include "arrow/table.h"
#include <iostream>

using namespace arrow;

struct RowBatchBox {
  std::shared_ptr<RowBatch> sp;
  RowBatch* p;
};

struct TableBox {
  std::shared_ptr<Table> sp;
  Table* p;
};

extern "C" {
  // RowBatch

  RowBatchBox* new_row_batch(SchemaBox* schema, int num_rows, ArrayBox* arrays[], int arr_len) {
    std::vector<std::shared_ptr<Array>> arr_vec;
    for (int i = 0; i < arr_len; i++) {
      arr_vec.push_back(arrays[i]->sp);
    }

    RowBatchBox* row_batch = new RowBatchBox;
    row_batch->sp = std::make_shared<RowBatch>(schema->sp, num_rows, arr_vec);
    row_batch->p = row_batch->sp.get();

    return row_batch;
  }

  void release_row_batch(RowBatchBox* row_batch) {
    if (row_batch) {
      delete row_batch;
    }
  }

  SchemaBox* row_batch_schema(RowBatchBox* row_batch) {
    SchemaBox* schema = new SchemaBox;
    schema->sp = row_batch->p->schema();
    schema->p = schema->sp.get();
    return schema;
  }

  ArrayBox* row_batch_column(RowBatchBox* row_batch, int i) {
    ArrayBox* arr = new ArrayBox;
    arr->sp = row_batch->p->column(i);
    arr->p = arr->sp.get();
    return arr;
  }

  const char* row_batch_col_name(RowBatchBox* row_batch, int i) {
    return row_batch->p->column_name(i).c_str();
  }

  int row_batch_num_cols(RowBatchBox* row_batch) {
    return row_batch->p->num_columns();
  }

  int row_batch_num_rows(RowBatchBox* row_batch) {
    return row_batch->p->num_rows();
  }

  // Table

}

#endif