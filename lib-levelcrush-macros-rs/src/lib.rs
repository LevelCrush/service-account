use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parser, parse_macro_input, ItemStruct};

/// appends standard fields related to a record
#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn DatabaseRecord(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        // created_at field
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub created_at: u64 })
                .unwrap(),
        );

        // updated_at field
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub updated_at: u64 })
                .unwrap(),
        );

        // deleted_at field
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub deleted_at: u64 })
                .unwrap(),
        );

        // deleted_at field
        fields.named.insert(
            0,
            syn::Field::parse_named
                .parse2(quote! { pub id: i64 })
                .unwrap(),
        );
    }

    quote! {
        #[derive(sqlx::FromRow, Debug, Default, Clone)]
        #item_struct
    }
    .into()
}
/// appends standard fields related to a record
#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn TimestampFields(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(item as ItemStruct);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        // created_at field
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub created_at: u64 })
                .unwrap(),
        );

        // updated_at field
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub updated_at: u64 })
                .unwrap(),
        );

        // deleted_at field
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub deleted_at: u64 })
                .unwrap(),
        );

        // deleted_at field
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub id: i32 })
                .unwrap(),
        );
    }

    quote! {
        #item_struct
    }
    .into()
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn DatabaseResult(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();

    let output = quote! {
        #[derive(sqlx::FromRow, Debug, Default, Clone)]
        #input
    };

    output.into()
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn DatabaseResultSerde(_args: TokenStream, input: TokenStream) -> TokenStream {
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
