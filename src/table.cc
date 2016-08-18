#include "table.h"
#include <cstring>

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
  std::string str = row_batch->p->column_name(i);
  char *cstr = new char[str.length() + 1];
  std::strcpy(cstr, str.c_str());
  return cstr;
}

int row_batch_num_cols(RowBatchBox* row_batch) {
  return row_batch->p->num_columns();
}

int row_batch_num_rows(RowBatchBox* row_batch) {
  return row_batch->p->num_rows();
}

// Table
TableBox* new_table(const char* name, SchemaBox* schema, ColumnBox* columns[], int num_cols) {
  std::vector<std::shared_ptr<Column>> col_vec;
  for (int i = 0; i < num_cols; i++) {
    col_vec.push_back(columns[i]->sp);
  }

  TableBox* table = new TableBox;
  table->sp = std::make_shared<Table>(std::string(name), schema->sp, col_vec);
  table->p = table->sp.get();
  return table;
}

void release_table(TableBox* table) {
  if (table) {
    delete table;
  }
}

const char* table_name(TableBox* table) {
  std::string str = table->p->name();
  char *cstr = new char[str.length() + 1];
  std::strcpy(cstr, str.c_str());
  return cstr;
}

SchemaBox* table_schema(TableBox* table) {
  SchemaBox* schema = new SchemaBox;
  schema->sp = table->p->schema();
  schema->p = schema->sp.get();
  return schema;
}

ColumnBox* table_column(TableBox* table, int i) {
  ColumnBox* column = new ColumnBox;
  column->sp = table->p->column(i);
  column->p = column->sp.get();

  return column;
}

int table_num_cols(TableBox* table) {
  return table->p->num_columns();
}

int64_t table_num_rows(TableBox* table) {
  return table->p->num_rows();
}

StatusBox* validate_table_cols(TableBox* table) {
  StatusBox* status = new StatusBox;
  status->status = table->p->ValidateColumns();
  return status;
}