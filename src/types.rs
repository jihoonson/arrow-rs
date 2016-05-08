
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

pub struct DataType<'a> {
  pub ty: Ty,
  pub children: Vec<*const Field<'a>>,
}

pub struct Field<'a> {
  pub name: &'a str,

}
