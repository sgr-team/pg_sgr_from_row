use proc_macro2::TokenStream as TokenStream2;
use syn::DeriveInput;
use quote::quote;

pub fn execute_macro(crate_name: TokenStream2, input: DeriveInput) -> TokenStream2 {
  let ident = input.ident;

  quote! {
    impl #ident {
      pub fn from_rows(rows: Vec<#crate_name::Row>) -> Vec<Self> {
        rows.iter().map(|x| x.into()).collect::<Vec<#ident>>() 
      }
    }
  }
}