mod fields;

use proc_macro2::TokenStream as TokenStream2;
use syn::DeriveInput;
use quote::quote;

pub fn execute_macro(crate_name: TokenStream2, input: DeriveInput) -> TokenStream2 {
  let struct_ident = input.ident.clone();

  let fields = match input.data {
    syn::Data::Struct(data) => 
      match fields::Fields::parse_struct(&struct_ident, data) {
        Ok(x) => x,
        Err(e) => panic!("{}: {}", struct_ident, e)
      },
    syn::Data::Enum(_) => panic!("FromRow [{}]: enums not supported", struct_ident),
    syn::Data::Union(_) => panic!("FromRow [{}]: unions not supported", struct_ident),
  };

  quote! {
    impl From<&#crate_name::Row> for #struct_ident {
      fn from(row: &#crate_name::Row) -> Self { 
        Self {
          #fields
        }
      }
    }
  }
}