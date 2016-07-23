#ifndef TABLE_H
#define TABLE_H

struct RowBatchBox {
  std::shared_ptr<RowBatch> sp;
  RowBatch* p;
};

struct TableBox {
  std::shared_ptr<Table> sp;
  Table* p;
};

extern "C" {

}

#endif