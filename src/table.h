#ifndef TABLE_H
#define TABLE_H

#include "ty.h"
#include "column.h"
#include "array.h"
#include "common/status.h"
#include "arrow/table.h"

using namespace arrow;

struct RowBatchBox {
  std::shared_ptr<RecordBatch> sp;
  RecordBatch* p;
};

struct TableBox {
  std::shared_ptr<Table> sp;
  Table* p;
};

extern "C" {
  // RowBatch

  RowBatchBox* new_row_batch(SchemaBox* schema, int num_rows, ArrayBox* arrays[], int arr_len);

  void release_row_batch(RowBatchBox* row_batch);

  SchemaBox* row_batch_schema(RowBatchBox* row_batch);

  ArrayBox* row_batch_column(RowBatchBox* row_batch, int i);

  const char* row_batch_col_name(RowBatchBox* row_batch, int i);

  int row_batch_num_cols(RowBatchBox* row_batch);

  int row_batch_num_rows(RowBatchBox* row_batch);

  // Table
  TableBox* new_table(const char* name, SchemaBox* schema, ColumnBox* columns[], int num_cols);

  void release_table(TableBox* table);

  const char* table_name(TableBox* table);

  SchemaBox* table_schema(TableBox* table);

  ColumnBox* table_column(TableBox* table, int i);

  int table_num_cols(TableBox* table);

  int64_t table_num_rows(TableBox* table);

  StatusBox* validate_table_cols(TableBox* table);
}

#endif