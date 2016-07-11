#ifndef MEMORY_POOL_H
#define MEMORY_POOL_H

#include <iostream>
#include <cassert>
#include "common/status.h"
#include "arrow/util/memory-pool.h"

using namespace arrow;

extern "C" {
  bool mem_alloc(MemoryPool * pool, uint8_t* buffer, int64_t size, StatusBox &status) {
    Status s = pool->Allocate(size, &buffer);
    status.status = s;
    return s.ok();
  }

  void mem_free(MemoryPool* pool, uint8_t* buffer, int64_t size) {
    return pool->Free(buffer, size);
  }

  int64_t num_bytes_alloc(MemoryPool* pool) {
    return pool->bytes_allocated();
  }

  MemoryPool* default_mem_pool() {
    MemoryPool* p2 = default_memory_pool();
    return p2;
  }
}

#endif