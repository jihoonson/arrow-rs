#ifndef TABLE_H
#define TABLE_H

#include "arrow/column.h"
#include "schema.h"

struct ColumnBox {
  std::shared_ptr<Column> sp;
  Column* p;
};

struct RowBatchBox {
  std::shared_ptr<RowBatch> sp;
  RowBatch* p;
}

struct TableBox {
  std::shared_ptr<Table> sp;
  Table* p;
}

extern "C" {
  
}

#endif