#ifndef MEMORY_H
#define MEMORY_H

#include "arrow/io/memory.h"
#include "arrow/io/interfaces.h"
#include "../common/status.h"
#include "../buffer.h"

using namespace arrow;
using namespace io;

struct MemorySourceBox {
  std::shared_ptr<MemoryMappedFile> sp;
  MemoryMappedFile* p;
};

extern "C" {
  MemorySourceBox* open_mmap_src(const char* path, FileMode::type mode);

  void release_mmap_src(MemorySourceBox* src);

  StatusBox* close_mmap_src(MemorySourceBox* src);

  BufferBox* read_at_mmap_src(MemorySourceBox* src, int64_t position, int64_t nbytes);

  StatusBox* write_mmap_src(MemorySourceBox* src, int64_t position, const uint8_t* data, int64_t nbytes);

  int64_t mmap_src_size(MemorySourceBox* src);
}

#endif