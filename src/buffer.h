#ifndef BUFFER_H
#define BUFFER_H

#include "arrow/util/buffer.h"
#include "common/status.h"

using namespace arrow;

struct BufferBox {
  std::shared_ptr<Buffer> sp;
  Buffer* p;
};

struct BufferBuilderBox {
  std::shared_ptr<BufferBuilder> sp;
  BufferBuilder* p;
};

extern "C" {

  bool buf_part_equals(BufferBox* buf1, BufferBox* buf2, int64_t nbytes);

  bool buf_equals(BufferBox* buf1, BufferBox* buf2);

  int64_t buf_capa(BufferBox* buf);

  int64_t buf_size(BufferBox* buf);

  const uint8_t* buf_data(BufferBox* buf);

  // Mutable buffer

  uint8_t* buf_mut_data(BufferBox* buf);

  BufferBox* buf_immut_view(BufferBox* buf);

  // Pool buffer

  StatusBox* resize_buf(BufferBox* buf, int64_t new_size);

  StatusBox* reserve_buf(BufferBox* buf, int64_t new_capa);

  void release_buf(BufferBox* buf);

  // Buffer builder

  BufferBuilder* new_buf_builder(MemoryPool* pool);

  void release_buf_builder(BufferBuilder* builder);

  StatusBox* resize_buf_builder(BufferBuilder* builder, int32_t elements);

  StatusBox* raw_append_buf_builder(BufferBuilder* builder, const uint8_t* data, int length);

  BufferBox* finish_buf_builder(BufferBuilder* builder);

  int buf_builder_capa(BufferBuilder* builder);

  int buf_builder_len(BufferBuilder* builder);
}

#endif