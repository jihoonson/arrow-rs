#ifndef STATUS_H
#define STATUS_H

#include "arrow/util/status.h"

using namespace arrow;

struct StatusBox {
  Status status;
};

extern "C" {
//  StatusBox* new_status();

  void release_status(StatusBox *status);

  bool ok(StatusBox* status);

  bool is_oom(StatusBox* status);

  bool is_key_error(StatusBox* status);

  bool is_invalid(StatusBox* status);

  bool is_io_error(StatusBox* status);

  bool is_not_implemented(StatusBox* status);

  const char* status_to_str(StatusBox* status);

  const char* code_to_str(StatusBox* status);

  int16_t posix_code(StatusBox* status);

  StatusCode code(StatusBox* status);

  const char* message(StatusBox* status);
}

#endif