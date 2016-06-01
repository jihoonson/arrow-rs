#include "arrow/type.h"

using namespace arrow;

struct DataTypePtr {
  std::shared_ptr<DataType> ptr;
};

struct FieldPtr {
  std::shared_ptr<Field> ptr;
};

extern "C" {
  DataTypePtr* new_data_type(Type::type ty) {
    DataTypePtr* dt = new DataTypePtr;

    switch (ty) {
      case Type::INT32: {
        dt->ptr = std::make_shared<Int32Type>();
        break;
      }
      default: {
        dt->ptr = nullptr;
      }
    }

    return dt;
  }

  int value_size(DataTypePtr* dt) {
    return dt->ptr->value_size();
  }

  void release_data_type(DataTypePtr * dt) {
    if (dt) {
      delete dt;
    }
  }

  FieldPtr* new_field(char* name, DataTypePtr* data_type, bool nullable) {
    FieldPtr* fp = new FieldPtr;
    fp->ptr = std::make_shared<Field>(std::string(name), data_type->ptr, nullable);
    return fp;
  }

  const char* field_to_string(FieldPtr* fp) {
    return fp->ptr->ToString().c_str();
  }

  void release_field(FieldPtr* fp) {
    if (fp) {
      delete fp;
    }
  }
}
