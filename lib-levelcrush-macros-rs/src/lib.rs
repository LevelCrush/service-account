use proc_macro::TokenStream;
use quote::quote;
use std::env;
use syn::{parse::Parser, parse_macro_input, ItemStruct};

/// appends standard fields related to a record
#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn DatabaseRecord(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        // created_at field
        fields
            .named
            .push(syn::Field::parse_named.parse2(quote! { pub created_at: u64 }).unwrap());

        // updated_at field
        fields
            .named
            .push(syn::Field::parse_named.parse2(quote! { pub updated_at: u64 }).unwrap());

        // deleted_at field
        fields
            .named
            .push(syn::Field::parse_named.parse2(quote! { pub deleted_at: u64 }).unwrap());

        // deleted_at field
        fields
            .named
            .insert(0, syn::Field::parse_named.parse2(quote! { pub id: i64 }).unwrap());
    }

    quote! {
        #[derive(sqlx::FromRow, Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
        #item_struct
    }
    .into()
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn DatabaseResult(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();

    let output = quote! {
        #[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
        #input
    };

    output.into()
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn ExternalAPIResponse(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let output = quote! {
        #[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone)]
        #input
    };
    output.into()
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn StandardAPIResponse(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let output = quote! {
        #[derive(serde::Serialize, Debug, Clone)]
        #input
    };
    output.into()
}

/// The equivalent to include_str but it works relative to the project
#[proc_macro]
pub fn project_str(item: TokenStream) -> TokenStream {
    let input = item.to_string();
    // remove the first and last " characters
    let mut input = input.chars();
    input.next();
    input.next_back();
    let input = input.as_str();

    let base_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_default();

    let target = format!("{}/{}", base_dir, input);
    quote! {
      include_str!(#target)
    }
    .into()
}
