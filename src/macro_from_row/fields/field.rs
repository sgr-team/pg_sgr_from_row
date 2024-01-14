use proc_macro2::TokenStream as TokenStream2;
use syn::{
  punctuated::Punctuated,
  Ident, MetaNameValue, Token, Expr
};
use quote::{quote, ToTokens};

pub struct Field {
  pub ident: Ident,
  pub from: Option<Expr>,
  pub column: Option<Expr>,
  pub default: Option<Expr>,
}

impl Field {
  pub fn parse(field: &syn::Field) -> Result<Self, String> {
    let ident = match field.ident.as_ref() {
      Some(x) => x.clone(),
      None => return Err("field without ident".into())
    };
    let mut from = None;
    let mut column = None;
    let mut default = None;

    for attr in field.attrs.iter() {
      if !attr.path().is_ident("from_row") {
        continue;
      }

      let parsed: Punctuated<MetaNameValue, Token![,]> = attr
        .parse_args_with(Punctuated::parse_terminated)
        .map_err(|e| format!(r#"Parse field "{}" error: {:?}"#, ident, e))?;

      for name_value in parsed.into_iter() {
        if name_value.path.is_ident("from") {
          from = Some(name_value.value);
          continue;
        }

        if name_value.path.is_ident("column") {
          column = Some(name_value.value);
          continue;
        }

        if name_value.path.is_ident("default") {
          default = Some(name_value.value);
          continue;
        }

        return Err(format!(r#"Parse field "{}" error: unhandled attribute"#, ident)) 
      }
    }

    if from.is_some() && (column.is_some() || default.is_some()) {
      return Err(format!(r#"{} - "from" can't be used with other settings"#, ident));
    }

    Ok(Self { ident, from, column, default })
  }
}

impl ToTokens for Field {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let ident = self.ident.clone();
    let ident_str = ident.to_string();

    let mut column = quote! { #ident_str };

    if let Some(x) = self.from.clone() {
      quote! { #ident: #x, }.to_tokens(tokens);
      return;
    }

    if let Some(x) = self.column.clone() {
      column = quote! { #x };
    }

    if let Some(x) = self.default.clone() {
      quote! { #ident: row.get::<_, Option<_>>(#column).unwrap_or(#x), }.to_tokens(tokens);
      return;
    }

    quote! { #ident: row.get(#column), }.to_tokens(tokens);
  }
}