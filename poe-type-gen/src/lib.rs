use std::fs::File;
use std::io::Read;

use heck::SnakeCase;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2, TokenTree};
use quote::quote;
use serde_json::Value;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, DeriveInput, Field, Result,
};

#[derive(Debug)]
struct GenStructsInput {
    file: syn::LitStr,
}

impl Parse for GenStructsInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let file: syn::LitStr = input.parse()?;

        Ok(GenStructsInput { file })
    }
}

#[proc_macro]
pub fn genstructs(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as GenStructsInput);

    let _file: String = input.file.value();

    let mut file = File::open(_file).unwrap();
    let mut contents = std::vec::Vec::new();
    let _len = file.read_to_end(&mut contents).unwrap();

    let json: Value = serde_json::from_slice(&contents[..]).unwrap();

    let structs = json.as_object().unwrap().iter().map(|(k, v)| {
        let struc = struct_from_json(v, k);
        quote! {
            #struc
        }
    });

    let strucs = quote! {
        #(#structs)*
    };

    strucs.into()
}

#[proc_macro_derive(Parse, attributes(parse_as))]
pub fn parse_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = input.data
    {
        named
    } else {
        unimplemented!();
    };

    let parse_steps = fields.iter().map(|field| parser_for_field(field));

    let build_fields = fields.iter().map(|field| &field.ident);

    let parse_impl = quote! {

        impl poe_parser::Parse for #ident {
            fn parse<'a>(input: &'a [u8], variable_data: &'a [u8]) -> nom::IResult<&'a [u8], Self>
            where
                Self: Sized,
            {
                #(#parse_steps)*

                Ok((
                    input,
                    #ident {
                        #(#build_fields,)*
                    },
                ))
            }
        }
    };

    parse_impl.into()
}

fn struct_from_json(json: &Value, name: &str) -> TokenStream2 {
    if let Value::Object(map) = json {
        let id = syn::Ident::new(&name[..name.len() - 4], Span::call_site());

        let fields: std::vec::Vec<_> = map["fields"]
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, v)| field_for_json_field(k, v))
            .collect();

        quote! {
            #[derive(Debug, Parse, Serialize, Deserialize)]
            pub struct #id {
                #(#fields,)*
            }
        }
    } else {
        unimplemented!()
    }
}

fn field_for_json_field(k: &str, v: &Value) -> TokenStream2 {
    let k = k.replace("2DArt", "Art2D").replace("Type", "Typ");
    let key = syn::Ident::new(&k.to_snake_case()[..], Span::call_site());
    let ty = v["type"].as_str().unwrap();

    let typ = match ty {
        "bool" => quote! { r#bool },
        "byte" => quote! { r#u8 },
        "float" => quote! { r#f32 },
        "int" => quote! { r#i32 },
        "long" => quote! { r#i64 },
        "ref|generic" => quote! { r#i32 },
        "ref|int" => quote! { r#i32 },
        "ref|list|float" => quote! { std::vec::Vec<f32> },
        "ref|list|int" => quote! { std::vec::Vec<i32> },
        "ref|list|long" => quote! { std::vec::Vec<i64> },
        "ref|list|ref|generic" => quote! { std::vec::Vec<i32> },
        "ref|list|ref|string" => quote! { std::vec::Vec<String> },
        "ref|list|uint" => quote! { std::vec::Vec<u32> },
        "ref|list|ulong" => quote! { std::vec::Vec<u64> },
        "ref|string" => quote! { r#String },
        "short" => quote! { r#i16 },
        "uint" => quote! { r#u32 },
        "ulong" => quote! { r#u64 },
        _ => unimplemented!(),
    };

    quote! {
        #[parse_as(#ty)]
        #key: #typ
    }
}

//TODO: THIS IS MUCH EASIER WITH THE parse_as TAG
fn parser_for_field(field: &Field) -> TokenStream2 {
    let name = &field.ident;
    let mut res = TokenStream2::new();

    'outer: for attr in field.attrs.clone().into_iter() {
        for token in attr.tokens.into_iter() {
            match token {
                TokenTree::Group(group) => {
                    for token in group.stream().into_iter() {
                        match token {
                            TokenTree::Literal(lit) => {
                                match &lit.to_string()[..] {
                                    "\"bool\"" => {
                                        res = quote! { let (input, temp) = nom::number::complete::le_u8(input)?; let #name = temp == 0x01; }
                                    }
                                    "\"byte\"" => {
                                        res = quote! { let (input, #name) = nom::number::complete::le_u8(input)?; }
                                    }
                                    "\"float\"" => {
                                        res = quote! { let (input, #name) = nom::number::complete::le_f32(input)?; }
                                    }
                                    "\"int\"" => {
                                        res = quote! { let (input, #name) = nom::number::complete::le_i32(input)?; }
                                    }
                                    "\"long\"" => {
                                        res = quote! { let (input, #name) = nom::number::complete::le_i64(input)?; }
                                    }
                                    "\"ref|generic\"" => {
                                        res = quote! { let (input, #name) = nom::number::complete::le_i32(input)?; }
                                    }
                                    "\"ref|int\"" => {
                                        res = quote! { let (input, #name) = nom::number::complete::le_i32(input)?; }
                                    }
                                    "\"ref|list|float\"" => {
                                        res = quote! { let (input, #name) = Self::parse_vec(input, variable_data, Box::new(|i, _| nom::number::complete::le_f32(i)))?; }
                                    }
                                    "\"ref|list|int\"" => {
                                        res = quote! { let (input, #name) = Self::parse_vec(input, variable_data, Box::new(|i, _| nom::number::complete::le_i32(i)))?; }
                                    }
                                    "\"ref|list|long\"" => {
                                        res = quote! { let (input, #name) = Self::parse_vec(input, variable_data, Box::new(|i, _| nom::number::complete::le_i64(i)))?; }
                                    }
                                    "\"ref|list|ref|generic\"" => {
                                        res = quote! { let (input, #name) = Self::parse_vec(input, variable_data, Box::new(|i, _| nom::number::complete::le_i32(i)))?; }
                                    }
                                    "\"ref|list|ref|string\"" => {
                                        res = quote! { let (input, #name) = Self::parse_vec(input, variable_data, Box::new(|i, v| Self::parse_ref_string(i, v)))?; }
                                    }
                                    "\"ref|list|uint\"" => {
                                        res = quote! { let (input, #name) = Self::parse_vec(input, variable_data, Box::new(|i, _| nom::number::complete::le_u32(i)))?; }
                                    }
                                    "\"ref|list|ulong\"" => {
                                        res = quote! { let (input, #name) = Self::parse_vec(input, variable_data, Box::new(|i, _| nom::number::complete::le_u64(i)))?; }
                                    }
                                    "\"ref|string\"" => {
                                        res = quote! { let (input, #name) = Self::parse_ref_string(input, variable_data)?; }
                                    }
                                    "\"short\"" => {
                                        res = quote! { let (input, #name) = nom::number::complete::le_i16(input)?; }
                                    }
                                    "\"uint\"" => {
                                        res = quote! { let (input, #name) = nom::number::complete::le_u32(input)?; }
                                    }
                                    "\"ulong\"" => {
                                        res = quote! { let (input, #name) = nom::number::complete::le_u64(input)?; }
                                    }
                                    _ => unimplemented!(),
                                }
                                break 'outer;
                            }
                            _ => unimplemented!(),
                        }
                    }
                }
                _ => unimplemented!(),
            }
        }
    }

    res
}
