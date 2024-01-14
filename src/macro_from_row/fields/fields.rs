use proc_macro2::TokenStream as TokenStream2;
use syn::{Ident, DataStruct};
use quote::{quote, ToTokens};

use super::Field;

pub struct Fields (Vec<Field>);

impl Fields {
  pub fn parse_struct(struct_ident: &Ident, data: DataStruct) -> Result<Self, String> {
    let mut fields = vec![];
    
    for field in data.fields.iter() {
      fields.push(Field::parse(field).map_err(|e| format!("{}: {}", struct_ident, e))?);
    }

    Ok(Self(fields))
  }
}

impl ToTokens for Fields {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let fields = self.0.iter();
    
    quote! { #(#fields)* }.to_tokens(tokens);
  }
}