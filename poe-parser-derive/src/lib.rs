use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Result};

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

#[proc_macro_derive(Parsable)]
pub fn genstructs(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as GenStructsInput);
    println!("{:?}", input.file.value());

    let _file: String = input.file.value();

    let stuff = quote! {};

    stuff.into()
}
