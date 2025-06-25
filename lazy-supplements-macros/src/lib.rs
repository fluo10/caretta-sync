use heck::ToUpperCamelCase;
use proc_macro::{self,  TokenStream};
use proc_macro2::Span;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Expr, ExprTuple, Field, Fields, FieldsNamed, Ident};

#[proc_macro_derive(SyncableModel)]
pub fn syncable_model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;
    assert_eq!(format_ident!("{}", struct_name), "Model");
    let fields = extract_fields(&input.data);
    let uuid_field = extract_uuid_field(&fields);
    let uuid_field_camel = Ident::new(&uuid_field.to_string().to_upper_camel_case(), Span::call_site());
    let timestamp_field = extract_timestamp_field(&fields);
    let timestamp_field_camel = Ident::new(&timestamp_field.to_string().to_upper_camel_case(), Span::call_site());
    let skip_fields = extract_skip_fields(&fields);
    let output = quote!{
        impl SyncableModel for #struct_name {
            type SyncableEntity = Entity;
            fn get_uuid(&self) -> Uuid {
                self.#uuid_field
            }
            fn get_timestamp() -> DateTimeUtc {
                self.#timestamp_field
            }
        }
        impl SyncableEntity for Entity {
            type SyncableModel = Model;
            type SyncableActiveModel = ActiveModel;
            type SyncableColumn = Column;
        }

        impl SyncableActiveModel for ActiveModel {
            type SyncableEntity = Entity;
            fn get_uuid(&self) -> Option<Uuid> {
                self.#uuid_field.into_value()
            }
            fn get_timestamp(&self) -> Option<DateTimeUtc> {
                self.#timestamp_field.into_value()
            }
        }
        impl SyncableColumn for Column {
            fn is_uuid(&self) -> bool {
                self == &Column::#uuid_field_camel 
            }
            fn is_timestamp(&self) -> bool {
                self == &Column::#timestamp_field_camel
            }
        }
    };
    output.into()
}
fn extract_skip_fields(fields: &FieldsNamed) -> Vec<&Ident> {
    extract_fields_with_attribute(fields, "skip")
}
fn extract_timestamp_field(fields: &FieldsNamed) -> &Ident {
    let mut timestamp_fields = extract_fields_with_attribute(fields, "timestamp");
    if timestamp_fields.len() == 1 {
        timestamp_fields.pop().unwrap()
    } else  {
        panic!("Model must need one timestamp field attribute")
    }
}
fn extract_uuid_field(fields: &FieldsNamed) -> &Ident {
    let mut uuid_fields = extract_fields_with_attribute(fields, "uuid");
    if uuid_fields.len() == 1 {
        uuid_fields.pop().unwrap()
    } else  {
        panic!("Model must need one uuid field attribute")
    }
}
fn extract_fields_with_attribute<'a>(fields: &'a FieldsNamed, attribute_arg: &'static str) -> Vec<&'a Ident>{
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

