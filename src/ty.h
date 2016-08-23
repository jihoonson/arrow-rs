#ifndef TYPE_H
#define TYPE_H

#include "arrow/type.h"
#include "arrow/schema.h"

using namespace arrow;

struct DataTypeBox {
  std::shared_ptr<DataType> sp;
  DataType* p;
};

struct FieldBox {
  std::shared_ptr<Field> sp;
  Field* p;
};

struct SchemaBox {
  std::shared_ptr<Schema> sp;
  Schema* p;
};

extern "C" {

  bool is_primitive_type(Type::type ty);

  DataTypeBox* new_primitive_type(Type::type ty);

  DataTypeBox* new_list_type(DataTypeBox* value_type);

  DataTypeBox* new_binary_type();

  DataTypeBox* new_string_type();

  DataTypeBox* new_struct_type(int field_num, FieldBox* fields []);

  bool data_type_equals(const DataTypeBox* dt1, const DataTypeBox* dt2);

  int value_size(DataTypeBox* dt);

  const char* data_type_to_string(DataTypeBox* dt);

  void release_data_type(DataTypeBox * dt);

  FieldBox* new_field(char* name, DataTypeBox* data_type, bool nullable);

  bool field_equals(const FieldBox* f1, const FieldBox* f2);

  const char* field_to_string(FieldBox* fp);

  void release_field(FieldBox* fp);

  SchemaBox* new_schema(int field_num, FieldBox* fields []);

  FieldBox* get_schema_field(SchemaBox* schema, int i);

  int schema_size(SchemaBox* schema);

  bool schema_equals(SchemaBox* s1, SchemaBox* s2);

  const char* schema_to_string(SchemaBox* schema);

  void release_schema(SchemaBox* schema);
}

#endif