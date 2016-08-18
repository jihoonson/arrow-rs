#include "status.h"
#include <cstring>

extern "C" {
//  StatusBox* new_status() {
//    StatusBox* box = new StatusBox;
//    box->status = Status::OK();
//    return box;
//  }

  void release_status(StatusBox* status) {
    delete status;
  }

  bool ok(StatusBox* status) {
    return status->status.ok();
  }

  bool is_oom(StatusBox* status) {
    return status->status.IsOutOfMemory();
  }

  bool is_key_error(StatusBox* status) {
    return status->status.IsKeyError();
  }

  bool is_invalid(StatusBox* status) {
    return status->status.IsInvalid();
  }

  bool is_io_error(StatusBox* status) {
    return status->status.IsIOError();
  }

  bool is_not_implemented(StatusBox* status) {
    return status->status.IsNotImplemented();
  }

  const char* status_to_str(StatusBox* status) {
    std::string str = status->status.ToString();
    char *cstr = new char[str.length() + 1];
    std::strcpy(cstr, str.c_str());
    return cstr;
  }

  const char* code_to_str(StatusBox* status) {
    std::string str = status->status.CodeAsString();
    char *cstr = new char[str.length() + 1];
    std::strcpy(cstr, str.c_str());
    return cstr;
  }

  int16_t posix_code(StatusBox* status) {
    return status->status.posix_code();
  }

  StatusCode code(StatusBox* status) {
    return status->status.code();
  }

  const char* message(StatusBox* status) {
    std::string str = status->status.message();
    char *cstr = new char[str.length() + 1];
    std::strcpy(cstr, str.c_str());
    return cstr;
  }
}