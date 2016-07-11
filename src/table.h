#ifndef TABLE_H
#define TABLE_H

#include "arrow/column.h"
#include "schema.h"

struct ColumnBox {
  std::shared_ptr<Column> sp;
  Column* column;
};

struct RowBatchBox {
  std::shared_ptr<RowBatch> sp;
  RowBatch* row_batch;
}

struct TableBox {
  std::shared_ptr<Table> sp;
  Table* table;
}

extern "C" {
  
}

#endif