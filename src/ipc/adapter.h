#ifndef ADAPTER_H
#define ADAPTER_H

#include "arrow/ipc/adapter.h"
#include "../common/status.h"
#include "memory.h"
#include "../table.h"
#include "../ty.h"

using namespace arrow;

struct RowBatchReaderBox {
  std::shared_ptr<ipc::RowBatchReader> sp;
  ipc::RowBatchReader* p;
};

extern "C" {
  int64_t write_row_batch(MemorySourceBox* src, const RowBatchBox* batch, int64_t position, int max_recursion_depth) {
    int64_t header_offset;
    Status status = ipc::WriteRowBatch(src->p, batch->p, position, &header_offset, max_recursion_depth);
//    assert(status.ok());

    return header_offset;
  }

  int64_t get_row_batch_size(RowBatchBox* batch) {
    int64_t size;
    Status status = ipc::GetRowBatchSize(batch->p, &size);
//    assert(status.ok());
    return size;
  }

  RowBatchReaderBox* open_row_batch_reader(MemorySourceBox* src, int64_t pos) {
    std::shared_ptr<RowBatchReader> sp;
    Status status = ipc::RowBatchReader::Open(src->p, pos, &sp);
//    assert(status.ok());

    RowBatchReaderBox* reader = new RowBatchReaderBox;
    reader->sp = sp;
    reader->p = sp.get();
    return reader;
  }

  void release_row_batch_reader(RowBatchReaderBox* reader) {
    if (reader) {
      delete reader;
    }
  }

  RowBatchBox* get_row_batch(RowBatchReaderBox* reader, SchemaBox* schema) {
    std::shared_ptr<RowBatch> sp;
    Status status = reader->p->GetRowBatch(schema->sp, &sp);
//    assert(status.ok());

    RowBatchBox* row_batch = new RowBatchBox;
    row_batch->sp = sp;
    row_batch->p = sp.get();
    return row_batch;
  }
}

#endif