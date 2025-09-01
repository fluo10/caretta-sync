mod derive;

use heck::ToUpperCamelCase;
use proc_macro::{self,  TokenStream};
use proc_macro2::Span;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Expr, ExprTuple, Field, Fields, FieldsNamed, Ident};
use derive::*;

fn extract_unique_field_ident<'a>(fields: &'a FieldsNamed, attribute_arg: &'static str) -> &'a Ident {
    let mut fields = extract_field_idents(fields, attribute_arg);
    if fields.len() == 1 {
        return fields.pop().unwrap()
    } else  {
        panic!("Model must need one {} field attribute", attribute_arg);
    };
}

fn extract_field_idents<'a>(fields: &'a FieldsNamed, attribute_arg: &'static str) -> Vec<&'a Ident>{
    fields.named.iter()
        .filter_map(|field| {
            field.attrs.iter()
                .find_map(|attr| {
                    if attr.path().is_ident("syncable") {
                        let args: Expr = attr.parse_args().unwrap();

                        match args {

                            Expr::Tuple(arg_tupple) => {
                                
                                arg_tupple.elems.iter()
                                .find_map(|arg| {
                                    if let Expr::Path(arg_path) = arg {
                                        if arg_path.path.is_ident(attribute_arg) {
                                            Some(field.ident.as_ref().unwrap())
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                })
                            },
                            Expr::Path(arg_path) => {
                                if arg_path.path.is_ident(attribute_arg) {
                                    Some(field.ident.as_ref().unwrap())
                                } else {
                                    None
                                }
                            },
                            _ => None
                        }
                    } else {
                        None
                    }
                })
        }).collect()
}

fn extract_fields(data: &Data) -> &FieldsNamed {
    match *data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => fields,
            _ => panic!("all fields must be named.")
        },
        _ => panic!("struct expected, but got other item."),
    }
}

#[proc_macro_derive(Emptiable)]
pub fn emptiable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let type_ident = input.ident;
    match input.data {
        Data::Struct(ref data) => {
            let field_idents = extract_idents_and_types_from_data_struct(data);
            let is_empty_iter = field_idents.iter().map(|(ident, type_name)| {
                quote!{
                    <#type_name as Emptiable>::is_empty(&self.#ident)
                }
            });
            let empty_iter = field_idents.iter().map(|(ident, type_name)| {
                quote!{
                    #ident: <#type_name as Emptiable>::empty(),
                }
            });
            quote!{
                impl Emptiable for #type_ident {
                    fn empty() -> Self {
                        Self {
                            #(#empty_iter)*
                        }
                    }
                    fn is_empty(&self) -> bool {
                        #(#is_empty_iter)&&*
                    }
                }
            }.into()
        }
        _ => panic!("struct or expected, but got other type.")

    }
}

#[proc_macro_derive(Mergeable)]
pub fn mergeable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let type_ident = input.ident;
    match input.data {
        Data::Struct(ref data) => {
            let field_idents = extract_idents_and_types_from_data_struct(data);
            let merge_iter = field_idents.iter().map(|(ident, type_name)| {
                quote!{
                    <#type_name as Mergeable>::merge(&mut self.#ident, other.#ident);
                }
            });
            quote!{
                impl Mergeable for #type_ident {
                    fn merge(&mut self, mut other: Self){
                        #(#merge_iter)*
                    }
                }
            }.into()
        }
        _ => panic!("struct expected, but got other type.")

    }
}

#[proc_macro_derive(Runnable, attributes(runnable))]
pub fn runnable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let type_ident = input.ident;
    match input.data {
        Data::Struct(ref data) => {
            let mut idents = extract_idents_and_types_from_data_struct_with_attribute(data, "runnable");
            let (field_ident, field_type) = unwrap_vec_or_panic(idents, "Runnable struct must have one field with runnable attribute");

            quote!{
                impl Runnable for #type_ident {
                    fn run(self, app_name: &'static str) {
                        <#field_type as Runnable>::run(self.#field_ident, app_name)                        
                    }
                }
            }.into()
        }
        Data::Enum(ref variants) => {
            let quote_vec = extract_idents_and_types_from_enum_struct(&variants);
            let quote_iter = quote_vec.iter().map(|(variant_ident, variant_type)|{
                quote!{
                    Self::#variant_ident(x) => <#variant_type as Runnable>::run(x, app_name),
                }
            });
            quote!{
                impl Runnable for #type_ident {
                    fn run(self, app_name: &'static str) {
                        match self {
                            #(#quote_iter)*
                        }
                    }
                }
            }.into()

        }, 
        _ => panic!("struct or enum expected, but got union.")

    }
}