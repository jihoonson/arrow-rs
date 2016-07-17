#include "memory_pool.h"

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