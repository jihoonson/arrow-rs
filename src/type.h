#include "arrow/type.h"

using namespace arrow;

struct DataTypeBox {
  std::shared_ptr<DataType> sp;
  DataType* dt;
};

struct FieldBox {
  std::shared_ptr<Field> sp;
  Field* field;
};

extern "C" {
  DataTypeBox* new_data_type(Type::type ty) {
    DataTypeBox* dt = new DataTypeBox;

    switch (ty) {
      case Type::INT32: {
        dt->sp = std::make_shared<Int32Type>();
        dt->dt = dt->sp.get();
        break;
      }
      default: {
        dt->dt = nullptr; // TODO: exception
      }
    }

    return dt;
  }

  int data_type_equals(const DataTypeBox* dt1, const DataTypeBox* dt2) {
    return dt1->dt->Equals(dt2->dt);
  }

  int value_size(DataTypeBox* dt) {
    return dt->dt->value_size();
  }

  const char* data_type_to_string(DataTypeBox* dt) {
    return dt->dt->ToString().c_str();
  }

  void release_data_type(DataTypeBox * dt) {
    if (dt) {
      delete dt;
    }
  }

  FieldBox* new_field(char* name, DataTypeBox* data_type, bool nullable) {
    FieldBox* fp = new FieldBox;
    fp->sp = std::make_shared<Field>(std::string(name), data_type->sp, nullable);
    fp->field = fp->sp.get();
    return fp;
  }

  int field_equals(const FieldBox* f1, const FieldBox* f2) {
    return f1->field->Equals(*(f2->field));
  }

  const char* field_to_string(FieldBox* fp) {
    return fp->field->ToString().c_str();
  }

  void release_field(FieldBox* fp) {
    if (fp) {
      delete fp;
    }
  }
}
