use proc_macro2::TokenStream as TokenStream2;
use syn::DeriveInput;
use quote::quote;

pub fn execute_macro(crate_name: TokenStream2, input: DeriveInput) -> TokenStream2 {
  let ident = input.ident;

  quote! {
    impl<'a> #crate_name::types::FromSql<'a> for #ident {
      fn from_sql(
        ty: &#crate_name::types::Type, 
        raw: &'a [u8]
      ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        serde_json::Value::from_sql(ty, raw)
          .map(|x| serde_json::from_value(x).unwrap())
      }
    
      fn accepts(ty: &#crate_name::types::Type) -> bool {
        serde_json::Value::accepts(ty)
      }
    }
  }
}