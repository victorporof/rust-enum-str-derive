/*
Copyright 2016 Mozilla
Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this file except in compliance with the License. You may obtain a copy of the
License at http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the
specific language governing permissions and limitations under the License.
*/

use quote;
use syn;

use heck::{CamelCase, KebabCase, MixedCase, ShoutySnakeCase, SnakeCase, TitleCase};

pub fn generate_impl(ast: &syn::DeriveInput, body: quote::Tokens) -> quote::Tokens {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn as_static_str(&self) -> &'static str {
                #body
            }
        }

        impl #impl_generics AsRef<str> for #name #ty_generics #where_clause {
            fn as_ref(&self) -> &str {
                self.as_static_str()
            }
        }
    }
}

pub fn expand_macro(ast: &syn::DeriveInput, case_type: &str) -> quote::Tokens {
    match ast.body {
        syn::Body::Struct(_) => panic!("Can't derive structs"),
        syn::Body::Enum(ref data) => expand_enum(ast, data, case_type)
    }
}

pub fn expand_enum(ast: &syn::DeriveInput, variants: &[syn::Variant], case_type: &str) -> quote::Tokens {
    let variants_iterator = variants.iter().map(|variant| {
        let struct_name = &ast.ident;
        let variant_name = &variant.ident;

        match variant.data {
            syn::VariantData::Unit => expand_match_enum_unit_variant(struct_name, variant_name, case_type),
            syn::VariantData::Tuple(_) => expand_match_enum_tuple_variant(struct_name, variant_name, case_type),
            syn::VariantData::Struct(_) => expand_match_enum_struct_variant(struct_name, variant_name, case_type)
        }
    });

    generate_impl(
        ast,
        quote! {
            match self {
                #( #variants_iterator )*
            }
        }
    )
}

pub fn expand_match_enum_unit_variant(struct_name: &syn::Ident, variant_name: &syn::Ident, case_type: &str) -> quote::Tokens {
    let ident = quote! { #struct_name::#variant_name };
    let variant_name_str = convert_case(&variant_name.to_string(), case_type);

    quote! {
        &#ident => {
            #variant_name_str
        },
    }
}

pub fn expand_match_enum_tuple_variant(struct_name: &syn::Ident, variant_name: &syn::Ident, case_type: &str) -> quote::Tokens {
    let ident = quote! { #struct_name::#variant_name };
    let variant_name_str = convert_case(&variant_name.to_string(), case_type);

    quote! {
        &#ident(..) => {
            #variant_name_str
        },
    }
}

pub fn expand_match_enum_struct_variant(struct_name: &syn::Ident, variant_name: &syn::Ident, case_type: &str) -> quote::Tokens {
    let ident = quote! { #struct_name::#variant_name };
    let variant_name_str = convert_case(&variant_name.to_string(), case_type);

    quote! {
        &#ident { .. } => {
            #variant_name_str
        },
    }
}

pub fn convert_case(src: &str, case_type: &str) -> String {
    match case_type {
        "CamelCase" => src.to_camel_case(),
        "KebabCase" => src.to_kebab_case(),
        "MixedCase" => src.to_mixed_case(),
        "ShoutySnakeCase" => src.to_shouty_snake_case(),
        "SnakeCase" => src.to_snake_case(),
        "TitleCase" => src.to_title_case(),
        _ => src.to_string()
    }
}
