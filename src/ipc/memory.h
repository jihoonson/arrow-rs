#ifndef MEMORY_H
#define MEMORY_H

#include "arrow/ipc/memory.h"
#include "../common/status.h"
#include "../buffer.h"

using namespace arrow;
using namespace ipc;

struct MemorySourceBox {
  std::shared_ptr<MemorySource> sp;
  MemorySource* p;
};

extern "C" {
  MemorySourceBox* open_mmap_src(const char* path, MemorySource::AccessMode mode);

  void release_mmap_src(MemorySourceBox* src);

  StatusBox* close_mmap_src(MemorySourceBox* src);

  BufferBox* read_at_mmap_src(MemorySourceBox* src, int64_t position, int64_t nbytes);

  StatusBox* write_mmap_src(MemorySourceBox* src, int64_t position, const uint8_t* data, int64_t nbytes);

  int64_t mmap_src_size(MemorySourceBox* src);
}

#endif