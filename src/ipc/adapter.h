#ifndef ADAPTER_H
#define ADAPTER_H

#include "arrow/ipc/adapter.h"
#include "../common/status.h"
#include "../io/memory.h"
#include "../table.h"
#include "../ty.h"

using namespace arrow;

struct ArrowResult {
  void* result;
  StatusBox* status;
};

struct RowBatchReaderBox {
  std::shared_ptr<ipc::RecordBatchReader> sp;
  ipc::RecordBatchReader* p;
};

extern "C" {
  int64_t write_row_batch(MemorySourceBox* src, const RowBatchBox* batch, int64_t position, int max_recursion_depth) {
    int64_t header_offset;
    Status status = ipc::WriteRecordBatch(batch->p->columns(), batch->p->num_rows(), src->p, &position, &header_offset, max_recursion_depth);
//    assert(status.ok());

    return header_offset;
  }

  int64_t get_row_batch_size(RowBatchBox* batch) {
    int64_t size;
    Status status = ipc::GetRecordBatchSize(batch->p, &size);
//    assert(status.ok());
    return size;
  }

  ArrowResult* open_row_batch_reader(MemorySourceBox* src, int64_t pos) {
    ArrowResult* result = new ArrowResult;
    result->status = new StatusBox;

    std::shared_ptr<ipc::RecordBatchReader> sp;
    result->status->status = ipc::RecordBatchReader::Open(src->p, pos, &sp);
//    assert(status.ok());

    if (result->status->status.ok()) {
      RowBatchReaderBox* reader = new RowBatchReaderBox;
      reader->sp = sp;
      reader->p = sp.get();
      result->result = reader;
    } else {
      result->result = nullptr;
    }

    return result;
  }

  void release_row_batch_reader(RowBatchReaderBox* reader) {
    if (reader) {
      delete reader;
    }
  }

  RowBatchBox* get_row_batch(RowBatchReaderBox* reader, SchemaBox* schema) {
    std::shared_ptr<RecordBatch> sp;
    Status status = reader->p->GetRecordBatch(schema->sp, &sp);
//    assert(status.ok());

    RowBatchBox* row_batch = new RowBatchBox;
    row_batch->sp = sp;
    row_batch->p = sp.get();
    return row_batch;
  }

  void release_arrow_result(ArrowResult* result) {
    if (result) {
      delete result;
    }
  }
}

#endif