#ifndef ADAPTER_H
#define ADAPTER_H

#include <arrow/ipc/adapter.h>
#include "common/status.h"

using namespace arrow;

extern "C" {
  StatusBox* write_row_batch(MemorySource* src) {

  }
}

#endif