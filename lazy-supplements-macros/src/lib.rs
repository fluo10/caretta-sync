use heck::ToUpperCamelCase;
use proc_macro::{self,  TokenStream};
use proc_macro2::Span;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Expr, ExprTuple, Field, Fields, FieldsNamed, Ident};

#[proc_macro_derive(SyncableModel, attributes(syncable))]
pub fn syncable_model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;
    assert_eq!(format_ident!("{}", struct_name), "Model");
    let fields = extract_fields(&input.data);
    let id_snake = extract_unique_field_ident(&fields, "id");
    let id_camel = Ident::new(&id_snake.to_string().to_upper_camel_case(), Span::call_site());
    let timestamp_snake = extract_unique_field_ident(&fields, "timestamp");
    let timestamp_camel = Ident::new(&timestamp_snake.to_string().to_upper_camel_case(), Span::call_site());
    let author_id_snake = extract_unique_field_ident(&fields, "author_id");
    let author_id_camel = Ident::new(&author_id_snake.to_string().to_upper_camel_case(), Span::call_site());
    let skips_snake = extract_field_idents(&fields, "skip");
    let output = quote!{
        impl SyncableModel for #struct_name {
            type SyncableEntity = Entity;
            fn get_id(&self) -> Uuid {
                self.#id_snake
            }
            fn get_timestamp(&self) -> DateTimeUtc {
                self.#timestamp_snake
            }
            fn get_author_id(&self) -> Uuid {
                self.#author_id_snake
            }
        }
        impl SyncableEntity for Entity {
            type SyncableModel = Model;
            type SyncableActiveModel = ActiveModel;
            type SyncableColumn = Column;
        }

        impl SyncableActiveModel for ActiveModel {
            type SyncableEntity = Entity;
            fn get_id(&self) -> Option<Uuid> {
                self.#id_snake.try_as_ref().cloned()
            }
            fn get_timestamp(&self) -> Option<DateTimeUtc> {
                self.#timestamp_snake.try_as_ref().cloned()
            }
            fn get_author_id(&self) -> Option<Uuid> {
                self.#author_id_snake.try_as_ref().cloned()
            }
        }
        impl SyncableColumn for Column {
            fn is_id(&self) -> bool {
                matches!(self, Column::#id_camel)
            }
            fn is_timestamp(&self) -> bool {
                matches!(self, Column::#timestamp_camel)                 
            }
            fn is_author_id(&self) -> bool {
                matches!(self, Column::#author_id_camel)
            }
            fn should_synced(&self) -> bool {
                todo!()
            }
            fn timestamp_after(timestamp: DateTimeUtc) -> sea_orm::sea_query::expr::SimpleExpr {
                Column::#timestamp_camel.gte(timestamp)
            }
            fn author_id_eq(author_id: Uuid) -> sea_orm::sea_query::expr::SimpleExpr {
                Column::#author_id_camel.eq(author_id)
            }

        }
    };
    output.into()
}
fn extract_unique_field_ident<'a>(fields: &'a FieldsNamed, attribute_arg: &'static str) -> &'a Ident {
    let mut fields = extract_field_idents(fields, attribute_arg);
    if fields.len() == 1 {
        return fields.pop().unwrap()
    } else  {
        panic!("Model must need one {} field attribute", attribute_arg);
    }
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

