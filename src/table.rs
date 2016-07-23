use ty;
use column;
use array;
use common::status;
use libc;

pub enum RowBatch {}
pub enum Table {}

extern "C" {
  // RowBatch
  pub fn new_row_batch(schema: *const ty::Schema, num_rows: i32, arrays: &[*const array::Array], arr_len: i32) -> *const RowBatch;
  pub fn release_row_batch(row_batch: *const RowBatch);
  pub fn row_batch_schema(row_batch: *const RowBatch) -> *const ty::Schema;
  pub fn row_batch_column(row_batch: *const RowBatch, i: i32) -> *const array::Array;
  pub fn row_batch_col_name(row_batch: *const RowBatch) -> *const libc::c_char;
  pub fn row_batch_num_cols(row_batch: *const RowBatch) -> i32;
  pub fn row_batch_num_rows(row_batch: *const RowBatch) -> i32;

  // Table
  pub fn new_table(name: *const libc::c_char, schema: *const ty::Schema, cols: &[*const column::Column], num_cols: i32) -> *const Table;
  pub fn release_table(table: *const Table);
  pub fn table_name(table: *const Table) -> *const libc::c_char;
  pub fn table_schema(table: *const Table) -> *const ty::Schema;
  pub fn table_column(table: *const Table, i: i32) -> *const column::Column;
  pub fn table_num_cols(table: *const Table) -> i32;
  pub fn table_num_rows(table: *const Table) -> i64;
  pub fn validate_table_cols(table: *const Table) -> *const status::Status;
}