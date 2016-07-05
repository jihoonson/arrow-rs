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
  DataTypeBox* new_primitive_type(Type::type ty) {

    std::shared_ptr<DataType> sp;
    switch (ty) {
      case Type::NA: {
        sp = std::make_shared<NullType>();
        break;
      }
      case Type::BOOL: {
        sp = std::make_shared<BooleanType>();
        break;
      }
      case Type::UINT8: {
        sp = std::make_shared<UInt8Type>();
        break;
      }
      case Type::INT8: {
        sp = std::make_shared<Int8Type>();
        break;
      }
      case Type::UINT16: {
        sp = std::make_shared<UInt16Type>();
        break;
      }
      case Type::INT16: {
        sp = std::make_shared<Int16Type>();
        break;
      }
      case Type::UINT32: {
        sp = std::make_shared<UInt32Type>();
        break;
      }
      case Type::INT32: {
        sp = std::make_shared<Int32Type>();
        break;
      }
      case Type::UINT64: {
        sp = std::make_shared<UInt64Type>();
        break;
      }
      case Type::INT64: {
        sp = std::make_shared<Int64Type>();
        break;
      }
      case Type::FLOAT: {
        sp = std::make_shared<FloatType>();
        break;
      }
      case Type::DOUBLE: {
        sp = std::make_shared<DoubleType>();
        break;
      }
//      case Type::LIST: {
//        sp = std::make_shared<ListType>();
//        break;
//      }
//      case Type::BINARY: {
//        sp = std::make_shared<BinaryType>();
//        break;
//      }
//      case Type::STRING: {
//        sp = std::make_shared<StringType>();
//        break;
//      }
//      case Type::STRUCT: {
//        sp = std::make_shared<StructType>();
//        break;
//      }
      default: {
        return nullptr; // TODO: exception
      }
    }

    DataTypeBox* dt = new DataTypeBox;
    dt->sp = sp;
    dt->dt = sp.get();
    return dt;
  }

  DataTypeBox* new_list_type(DataTypeBox* value_type) {
    DataTypeBox* box = new DataTypeBox;
    box->sp = std::make_shared<ListType>(value_type->sp);
    box->dt = box->sp.get();
    return box;
  }

  DataTypeBox* new_binary_type() {
    DataTypeBox* box = new DataTypeBox;
    box->sp = std::make_shared<BinaryType>();
    box->dt = box->sp.get();
    return box;
  }

  DataTypeBox* new_string_type() {
    DataTypeBox* box = new DataTypeBox;
    box->sp = std::make_shared<StringType>();
    box->dt = box->sp.get();
    return box;
  }

  DataTypeBox* new_struct_type(int field_num, FieldBox* fields []) {
    std::vector<std::shared_ptr<Field>> vec;
    for (int i = 0; i < field_num; i++) {
      vec.push_back(fields[i]->sp);
    }
    DataTypeBox* box = new DataTypeBox;
    box->sp = std::make_shared<StructType>(vec);
    box->dt = box->sp.get();
    return box;
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
