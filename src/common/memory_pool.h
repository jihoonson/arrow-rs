#ifndef MEMORY_POOL_H
#define MEMORY_POOL_H

#include <iostream>
#include <cassert>
#include "status.h"
#include "arrow/util/memory-pool.h"

using namespace arrow;

extern "C" {
  bool mem_alloc(MemoryPool * pool, uint8_t* buffer, int64_t size, StatusBox &status);

  void mem_free(MemoryPool* pool, uint8_t* buffer, int64_t size);

  int64_t num_bytes_alloc(MemoryPool* pool);

  MemoryPool* default_mem_pool();
}

#endif