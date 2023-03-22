extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(Filterable)]
pub fn filterable_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_filterable(&ast)
}

fn impl_filterable(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let struct_name = &ast.ident;
    let struct_name_str = struct_name.to_string();
    let properties_name = syn::Ident::new(
        &format!("{}Properties", &struct_name_str),
        struct_name.span(),
    );

    let fields = match &ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("vec_filter can only be derived for structs with named fields"),
    };

    let mut property_variants = Vec::new();
    let mut property_variants_lower_str = Vec::new();
    let mut get_property_value_match_arms = Vec::new();
    let mut get_property_enum_match_arms = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_name_str = field_name.to_string();
        let field_name_str_lower = field_name_str.to_lowercase();
        property_variants_lower_str.push(field_name_str_lower.clone());

        let variant_ident = syn::Ident::new(&field_name_str, field_name.span());
        let property_variant = quote! { #variant_ident };
        property_variants.push(property_variant);

        let field_value = quote! { vec_filter::Value::wrap(self.#field_name.clone()) };
        let match_arm = quote! { #properties_name::#variant_ident => Some(#field_value), };
        get_property_value_match_arms.push(match_arm);

        let property_enum_match_arm =
            quote! { #field_name_str_lower => Ok(#properties_name::#variant_ident), };
        get_property_enum_match_arms.push(property_enum_match_arm);
    }

    let gen = quote! {
        #[allow(non_camel_case_types)]
        #[derive(Debug, PartialEq, Clone)]
        pub enum #properties_name {
            #(#property_variants),*
        }

        impl vec_filter::StructProperties for #properties_name {
            fn valid_fields() -> Vec<&'static str> {
                vec![
                    #(#property_variants_lower_str),*
                ]
            }
        }

        impl std::str::FromStr for #properties_name {
            type Err = vec_filter::ParsePropertyFromString;

            fn from_str(property_string: &str) -> Result<Self, vec_filter::ParsePropertyFromString> {
                match property_string.to_lowercase().as_str() {
                    #(#get_property_enum_match_arms)*
                    _ => Err(vec_filter::ParsePropertyFromString::ItemNotFound)
                }
            }
        }

        impl vec_filter::vec_filter<#properties_name> for #name {
            fn get_property_value(&self, property: &#properties_name) -> Option<vec_filter::Value> {
                match property {
                    #(#get_property_value_match_arms)*
                    _ => None,
                }
            }
        }
    };

    gen.into()
}
