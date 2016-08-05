#include "buffer.h"

bool buf_part_equals(BufferBox* buf1, BufferBox* buf2, int64_t nbytes) {
  return buf1->p->Equals(*(buf2->p), nbytes);
}

bool buf_equals(BufferBox* buf1, BufferBox* buf2) {
  return buf1->p->Equals(*(buf2->p));
}

int64_t buf_capa(BufferBox* buf) {
  return buf->p->capacity();
}

int64_t buf_size(BufferBox* buf) {
  return buf->p->size();
}

const uint8_t* buf_data(BufferBox* buf) {
  return buf->p->data();
}

// Mutable buffer

uint8_t* buf_mut_data(BufferBox* buf) {
  // TODO: check the given buffer is mutable
  if (MutableBuffer* v = dynamic_cast<MutableBuffer*>(buf->p)) {
    return ((MutableBuffer*)buf->p)->mutable_data();
  } else {
    return nullptr;
  }
}

BufferBox* buf_immut_view(BufferBox* buf) {
  // TODO: check the given buffer is mutable
  BufferBox* immut = new BufferBox;
  immut->sp = ((MutableBuffer*)buf->p)->GetImmutableView();
  immut->p = immut->sp.get();
  return immut;
}

// Pool buffer

StatusBox* resize_buf(BufferBox* buf, int64_t new_size) {
  // TODO: check the given buffer is a pool buffer
  StatusBox *s = new StatusBox;
  s->status = ((PoolBuffer*)buf->p)->Resize(new_size);
  return s;
}

StatusBox* reserve_buf(BufferBox* buf, int64_t new_capa) {
  // TODO: check the given buffer is a pool buffer
  StatusBox *s = new StatusBox;
  s->status = ((PoolBuffer*)buf->p)->Reserve(new_capa);
  return s;
}

void release_buf(BufferBox* buf) {
  if (buf) {
    delete buf;
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
  StatusBox *s = new StatusBox;
  s->status = builder->Resize(elements);
  return s;
}

StatusBox* raw_append_buf_builder(BufferBuilder* builder, const uint8_t* data, int length) {
  StatusBox *s = new StatusBox;
  s->status = builder->Append(data, length);
  return s;
}

BufferBox* finish_buf_builder(BufferBuilder* builder) {
  BufferBox* buf = new BufferBox;
  buf->sp = builder->Finish();
  buf->p = buf->sp.get();
  return buf;
}

int buf_builder_capa(BufferBuilder* builder) {
  return builder->capacity();
}

int buf_builder_len(BufferBuilder* builder) {
  return builder->length();
}