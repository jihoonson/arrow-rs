#ifndef BUFFER_H
#define BUFFER_H

#include "arrow/util/buffer.h"
#include "common/status.h"

using namespace arrow;

struct BufferBox {
  std::shared_ptr<Buffer> sp;
  Buffer* buf;
};

struct BufferBuilderBox {
  std::shared_ptr<BufferBuilder> sp;
  BufferBuilder* builder;
};

extern "C" {

  bool buf_part_equals(BufferBox* box1, BufferBox* box2, int64_t nbytes) {
    return box1->buf->Equals(*(box2->buf), nbytes);
  }

  bool buf_equals(BufferBox* box1, BufferBox* box2) {
    return box1->buf->Equals(*(box2->buf));
  }

  int64_t buf_capa(BufferBox* box) {
    return box->buf->capacity();
  }

  int64_t buf_size(BufferBox* box) {
    return box->buf->size();
  }

  const uint8_t* buf_data(BufferBox* box) {
    return box->buf->data();
  }

  // Mutable buffer

  uint8_t* buf_mut_data(BufferBox* box) {
    // TODO: check the given buffer is mutable
    return ((MutableBuffer*)box->buf)->mutable_data();
  }

  BufferBox* buf_immut_view(BufferBox* box) {
    // TODO: check the given buffer is mutable
    BufferBox* immut_box = new BufferBox;
    immut_box->sp = ((MutableBuffer*)box->buf)->GetImmutableView();
    immut_box->buf = immut_box->sp.get();
    return immut_box;
  }

  // Pool buffer

  StatusBox* resize_buf(BufferBox* box, int64_t new_size) {
    // TODO: check the given buffer is a pool buffer
    StatusBox *s = new StatusBox;
    s->status = ((PoolBuffer*)box->buf)->Resize(new_size);
    return s;
  }

  StatusBox* reserve_buf(BufferBox* box, int64_t new_capa) {
    // TODO: check the given buffer is a pool buffer
    StatusBox *s = new StatusBox;
    s->status = ((PoolBuffer*)box->buf)->Reserve(new_capa);
    return s;
  }

  void release_buf(BufferBox* box) {
    if (box) {
      delete box;
    }
  }

  // Buffer builder

  BufferBuilder* new_buf_builder(MemoryPool* pool) {
    BufferBuilder* builder = new BufferBuilder(pool);
    return builder;
  }

  void release_buf_builder(BufferBuilder* builder) {
    if (builder) {
      delete builder;
    }
  }

  StatusBox* resize_buf_builder(BufferBuilder* builder, int32_t elements) {
    StatusBox *box = new StatusBox;
    box->status = builder->Resize(elements);
    return box;
  }

  StatusBox* raw_append_buf_builder(BufferBuilder* builder, const uint8_t* data, int length) {
    StatusBox *box = new StatusBox;
    box->status = builder->Append(data, length);
    return box;
  }

  BufferBox* finish_buf_builder(BufferBuilder* builder) {
    BufferBox* box = new BufferBox;
    box->sp = builder->Finish();
    box->buf = box->sp.get();
    return box;
  }

  int buf_builder_capa(BufferBuilder* builder) {
    return builder->capacity();
  }

  int buf_builder_len(BufferBuilder* builder) {
    return builder->length();
  }
}

#endif