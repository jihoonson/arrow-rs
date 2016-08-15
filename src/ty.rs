use libc;
use std::ops::Drop;
use std::cmp::Eq;
use std::ffi::{CStr, CString};
#[macro_use]
use common;

// Data types in this library are all *logical*. They can be expressed as
// either a primitive physical type (bytes or bits of some fixed size), a
// nested type consisting of other data types, or another data type (e.g. a
// timestamp encoded as an int64)
// See arrow::Type
#[repr(C)]
pub enum Ty {
  // A degenerate NULL type represented as 0 bytes/bits
  NA = 0,

  // A boolean value represented as 1 bit
  BOOL = 1,

  // Little-endian integer types
  UINT8 = 2,
  INT8 = 3,
  UINT16 = 4,
  INT16 = 5,
  UINT32 = 6,
  INT32 = 7,
  UINT64 = 8,
  INT64 = 9,

  // 4-byte floating point value
  FLOAT = 10,

  // 8-byte floating point value
  DOUBLE = 11,

  // CHAR(N): fixed-length UTF8 string with length N
  CHAR = 12,

  // UTF8 variable-length string as List<Char>
  STRING = 13,

  // VARCHAR(N): Null-terminated string type embedded in a CHAR(N + 1)
  VARCHAR = 14,

  // Variable-length bytes (no guarantee of UTF8-ness)
  BINARY = 15,

  // By default, int32 days since the UNIX epoch
  DATE = 16,

  // Exact timestamp encoded with int64 since UNIX epoch
  // Default unit millisecond
  TIMESTAMP = 17,

  // Timestamp as double seconds since the UNIX epoch
  TIMESTAMP_DOUBLE = 18,

  // Exact time encoded with int64, default unit millisecond
  TIME = 19,

  // Precision- and scale-based decimal type. Storage type depends on the
  // parameters.
  DECIMAL = 20,

  // Decimal value encoded as a text string
  DECIMAL_TEXT = 21,

  // A list of some logical data type
  LIST = 30,

  // Struct of logical types
  STRUCT = 31,

  // Unions of logical types
  DENSE_UNION = 32,
  SPARSE_UNION = 33,

  // Union<Null, Int32, Double, String, Bool>
  JSON_SCALAR = 50,

  // User-defined type
  USER = 60
}

pub struct DataType {
  raw_type: RawDataTypePtr
}

pub struct Field {
  raw_field: RawFieldPtr
}

pub struct Schema {
  raw_schema: RawSchemaPtr
}

impl DataType {
  pub fn new(raw_type: RawDataTypePtr) -> DataType {
    DataType {
      raw_type: raw_type
    }
  }

  pub fn new_primitive(ty: Ty) -> DataType {
    DataType {
      raw_type: unsafe { new_primitive_type(ty) }
    }
  }

  pub fn new_list(elem_type: DataType) -> DataType {
    unsafe {
      DataType {
        raw_type: new_list_type(elem_type.raw_type)
      }
    }
  }

  pub fn new_binary() -> DataType {
    unsafe {
      DataType {
        raw_type: new_binary_type()
      }
    }
  }

  pub fn new_string() -> DataType {
    unsafe {
      DataType {
        raw_type: new_string_type()
      }
    }
  }

  pub fn new_struct(field_num: i32, fields: &[Field]) -> DataType {
    let raw_fields: Vec<RawFieldPtr> = fields.into_iter().map(|f| f.raw_field).collect::<Vec<RawFieldPtr>>();
    unsafe {
      DataType {
        raw_type: new_struct_type(field_num, raw_fields.as_slice())
      }
    }
  }

  pub fn value_size(&self) -> i32 {
    unsafe {
      value_size(self.raw_type)
    }
  }

  pub fn raw_data_type(&self) -> RawDataTypePtr {
    self.raw_type
  }
}

impl PartialEq for DataType {
  fn eq(&self, other: &DataType) -> bool {
    unsafe {
      data_type_equals(self.raw_type, other.raw_type)
    }
  }
}

impl ToString for DataType {
  fn to_string(&self) -> String {
    cstr_to_string!( unsafe { data_type_to_string(self.raw_type) } )
  }
}

impl Drop for DataType {
  fn drop(&mut self) {
    unsafe {
      release_data_type(self.raw_type);
    }
  }
}

impl Field {
  pub fn new(name: String, ty: DataType, nullable: bool) -> Field {
    Field {
      raw_field: unsafe { new_field(string_to_cstr!(name), ty.raw_type, nullable) }
    }
  }

  pub fn raw_field(&self) -> RawFieldPtr {
    self.raw_field
  }
}

impl PartialEq for Field {
  fn eq(&self, other: &Field) -> bool {
    unsafe {
      field_equals(self.raw_field, other.raw_field)
    }
  }
}

impl ToString for Field {
  fn to_string(&self) -> String {
    cstr_to_string!( unsafe { field_to_string(self.raw_field) } )
  }
}

impl Drop for Field {
  fn drop(&mut self) {
    unsafe {
      release_field(self.raw_field);
    }
  }
}

impl Schema {
  pub fn new(field_num: i32, fields: &[Field]) -> Schema {
    let raw_fields: Vec<RawFieldPtr> = fields.into_iter().map(|f| f.raw_field).collect::<Vec<RawFieldPtr>>();
    unsafe {
      Schema {
        raw_schema: new_schema(field_num, raw_fields.as_slice())
      }
    }
  }

  pub fn from_raw(raw_schema: RawSchemaPtr) -> Schema {
    Schema {
      raw_schema: raw_schema
    }
  }

  pub fn raw_schema(&self) -> RawSchemaPtr {
    self.raw_schema
  }
}

impl PartialEq for Schema {
  fn eq(&self, other: &Schema) -> bool {
    unsafe {
      schema_equals(self.raw_schema, other.raw_schema)
    }
  }
}

impl ToString for Schema {
  fn to_string(&self) -> String {
    cstr_to_string!( unsafe { schema_to_string(self.raw_schema) } )
  }
}

impl Drop for Schema {
  fn drop(&mut self) {
    unsafe {
      release_schema(self.raw_schema);
    }
  }
}

pub enum RawDataType {}
pub enum RawField {}
pub enum RawSchema {}

pub type RawDataTypePtr = *const RawDataType;
pub type RawFieldPtr = *const RawField;
pub type RawSchemaPtr = *const RawSchema;

// TODO: singleton instances of types

extern "C" {
  pub fn new_primitive_type(ty: Ty) -> RawDataTypePtr;
  pub fn new_list_type(data_type: RawDataTypePtr) -> RawDataTypePtr;
  pub fn new_binary_type() -> RawDataTypePtr;
  pub fn new_string_type() -> RawDataTypePtr;
  pub fn new_struct_type(field_num: i32, fields: &[RawFieldPtr]) -> RawDataTypePtr;

  pub fn data_type_equals(data_type1: RawDataTypePtr, data_type2: RawDataTypePtr) -> bool;
  pub fn value_size(data_type: RawDataTypePtr) -> i32;
  pub fn data_type_to_string(data_type: RawDataTypePtr) -> *const libc::c_char;
  pub fn release_data_type(data_type: RawDataTypePtr);

  pub fn new_field(name: *const libc::c_char, data_type: RawDataTypePtr, nullable: bool) -> RawFieldPtr;
  pub fn field_equals(field1: RawFieldPtr, field2: RawFieldPtr) -> bool;
  pub fn field_to_string(field: RawFieldPtr) -> *const libc::c_char;
  pub fn release_field(field: RawFieldPtr);

  pub fn new_schema(field_num: i32, fields: &[RawFieldPtr]) -> RawSchemaPtr;
  pub fn schema_equals(s1: RawSchemaPtr, s2: RawSchemaPtr) -> bool;
  pub fn schema_to_string(schema: RawSchemaPtr) -> *const libc::c_char;
  pub fn release_schema(schema: RawSchemaPtr);
}
