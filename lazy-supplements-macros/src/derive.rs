use syn::{DataEnum, DataStruct, Fields, FieldsNamed, Ident, Type};

fn extract_fields_named_from_data_struct(data: &DataStruct) -> &FieldsNamed {
    match data.fields {
        Fields::Named(ref fields) => fields,
        _ => panic!("all fields must be named.")
    }
}

fn extract_idents_from_fields_named_with_attribute<'a>(fields: &'a FieldsNamed, attribute: &'static str) -> Vec<&'a Ident>{
    fields.named.iter()
        .filter_map(|field| {
            field.attrs.iter()
                .find_map(|attr| {
                    if attr.path().is_ident(attribute) {
                        field.ident.as_ref()
                    } else {
                        None
                    }
                })
        }).collect()
}

pub fn extract_idents_and_types_from_data_struct_with_attribute<'a>(data: &'a DataStruct, attribute: &'static str) -> Vec<(Ident, Type)>{
    let fields = extract_fields_named_from_data_struct(data);
    fields.named.iter().filter_map(|field| {
        field.attrs.iter()
            .find_map(|attr| {
                if attr.path().is_ident(attribute) {
                    Some((field.ident.clone().unwrap(), field.ty.clone()))
                } else {
                    None
                }
            })
    }).collect()
}

pub fn extract_idents_and_types_from_data_struct<'a>(data: &'a DataStruct) -> Vec<(Ident, Type)>{
    let fields = extract_fields_named_from_data_struct(data);
    fields.named.iter().map(|x| {(x.ident.clone().unwrap(), x.ty.clone())}).collect()
}

pub fn unwrap_vec_or_panic<T>(mut source: Vec<T>, msg: &'static str) -> T {
    if source.len() == 1 {
        source.pop().unwrap()
    } else {
        panic!("{}", msg)
    }
}

pub fn extract_idents_and_types_from_enum_struct<'a>(data: &'a DataEnum) -> Vec<(Ident, Type)> {
    data.variants.iter().map(|variant| {
        let mut fields: Vec<Type> = match &variant.fields {
            Fields::Unnamed(fields_unnamed) => {
                fields_unnamed.unnamed.iter().map(|x| {
                    x.ty.clone()
                }).collect()
            }, 
            _ => panic!("Fields of enum variant must be unnamed!")
        };
        if fields.len() == 1 {
            (variant.ident.clone(), fields.pop().unwrap()) 
        } else {
            panic!("Fields of enum variant must be single!")
        }
    }).collect()
}
