//! # FromRow
//! 
//! Part of pg_sgr: macros for deserializing rust data structures from postgres rows.
//! 
//! ## Install
//! 
//! ### Sync
//! 
//! ```bash
//! cargo add pg_sgr --features sync,from_row
//! ```
//! 
//! ### Tokio
//! 
//! ```bash
//! cargo add pg_sgr --features tokio,from_row
//! ```
//! 
//! ## Macros
//! 
//! Crate contains 3 derive macros:
//! 
//! - [FromRow](derive.FromRow.html) - for deserializing rust data structures from &postgres::Row or &tokio_postgres::Row
//! - [FromRows](derive.FromRows.html) - for deserializing rust data structures from Vec\<postgres::Row\> or Vec\<tokio_postgres::Row\>
//! - [FromJson](derive.FromJson.html) - for deserializing rust data structures from JSON & JSONB columns
#[cfg(all(feature = "sync", feature = "tokio"))]
compile_error!(r#"feature "sync" and feature "tokio" cannot be both enabled at the same time"#);

#[cfg(all(not(feature = "sync"), not(feature = "tokio")))]
compile_error!(r#"one of the "sync" and "tokio" features must be enabled"#);

mod macro_from_json;
mod macro_from_row;
mod macro_from_rows;

use syn::{parse_macro_input, DeriveInput};
use proc_macro::TokenStream;
use quote::quote;

macro_rules! crate_name {
  () => {{
    #[cfg(feature = "sync")]
    quote! { postgres }

    #[cfg(feature = "tokio")]
    quote! { tokio_postgres }
  }}
}

/// # Description
/// 
/// The derive macro allows to deserilize rust data structure from &postgres::Row
/// 
/// # Attributes
/// 
/// Each field can be described by the from_row attribute.
/// 
/// Attribute supports 3 properties:
/// 
/// - from - specifies the expression used to obtain data from a row (variable "row" available)
/// - column - allows to rename a column
/// - default - allows to specify a default value for nullable columns
/// 
/// # Example
/// 
/// ```rust
/// use pg_sgr_from_row::FromRow;
/// 
/// #[derive(FromRow)]
/// struct Book {
///   pub isbn: String,
///   pub name: String,
///   #[from_row(from = 0)]
///   pub count: i32,
///   #[from_row(column = "bookDescription", default = "".into())]
///   pub description: String,
/// }
/// 
/// fn get_book_by_id(client: &mut postgres::Client, id: &str) -> Result<Book, postgres::Error> {
///   let row = client.query_one("SQL", &[ &id ])?;
///   Ok((&row).into())
/// }
/// ```
#[proc_macro_derive(FromRow, attributes(from_row))]
pub fn from_row_derive(input: TokenStream) -> TokenStream {
  TokenStream::from(
    macro_from_row::execute_macro(
      crate_name!(), 
      parse_macro_input!(input as DeriveInput)
    )
  )
}

/// # Description
/// 
/// The derive macro allows to deserilize rust data structure from Vec\<postgres::Row\>
/// 
/// # Example
/// 
/// ```rust
/// use pg_sgr_from_row::{FromRow, FromRows};
/// 
/// #[derive(FromRow, FromRows)]
/// struct Book {
///   pub isbn: String,
///   pub name: String,
/// }
/// 
/// fn get_books(client: &mut postgres::Client) -> Result<Vec<Book>, postgres::Error> {
///   client.query("SQL", &[]).map(Book::from_rows)
/// }
/// ```
#[proc_macro_derive(FromRows)]
pub fn from_rows_derive(input: TokenStream) -> TokenStream {
  TokenStream::from(
    macro_from_rows::execute_macro(
      crate_name!(), 
      parse_macro_input!(input as DeriveInput)
    )
  )
}

/// # Description
/// 
/// The derive macro allows to deserilize rust data structure from JSON or JSONB
/// 
/// # Example
/// 
/// ```rust
/// use pg_sgr_from_row::{FromRow, FromJson};
/// 
/// use serde::Deserialize;
/// 
/// #[derive(FromRow)]
/// struct Book {
///   pub isbn: String,
///   pub name: String,
///   pub author: Author,
/// }
/// 
/// #[derive(Deserialize, FromJson)]
/// #[serde(rename_all = "camelCase")]
/// struct Author {
///   pub first_name: String,
///   pub last_name: String,
/// }
/// 
/// fn get_book_by_id(client: &mut postgres::Client, id: &str) -> Result<Book, postgres::Error> {
///   let row = client.query_one("SQL", &[ &id ])?;
///   Ok((&row).into())
/// }
/// ```
#[proc_macro_derive(FromJson)]
pub fn from_json_derive(input: TokenStream) -> TokenStream {
  TokenStream::from(
    macro_from_json::execute_macro(
      crate_name!(), 
      parse_macro_input!(input as DeriveInput)
    )
  )
}