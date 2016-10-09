#include "memory.h"

MemorySourceBox* open_mmap_src(const char* path, FileMode::type mode) {
  std::shared_ptr<MemoryMappedFile> out;
  Status s = MemoryMappedFile::Open(std::string(path), mode, &out);
//  assert(s.ok());

  MemorySourceBox* mm_src = new MemorySourceBox;
  mm_src->sp = out;
  mm_src->p = mm_src->sp.get();
  return mm_src;
}

void release_mmap_src(MemorySourceBox* src) {
  if (src) {
    delete src;
  }
}

StatusBox* close_mmap_src(MemorySourceBox* src) {
  StatusBox* status = new StatusBox;
  status->status = src->p->Close();
  return status;
}

BufferBox* read_at_mmap_src(MemorySourceBox* src, int64_t position, int64_t nbytes) {
  std::shared_ptr<Buffer> sp;
  Status s = src->p->ReadAt(position, nbytes, &sp);
//  assert(s.ok());

  BufferBox* buf = new BufferBox;
  buf->sp = sp;
  buf->p = sp.get();
  return buf;
}

StatusBox* write_mmap_src(MemorySourceBox* src, int64_t position, const uint8_t* data, int64_t nbytes) {
  StatusBox* status = new StatusBox;
  status->status = src->p->WriteAt(position, data, nbytes);
  return status;
}

int64_t mmap_src_size(MemorySourceBox* src) {
  int64_t size;
  Status status = src->p->GetSize(&size);
  return size;
}