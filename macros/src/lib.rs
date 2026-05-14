use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Expr, Ident, LitInt, Result, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

#[proc_macro]
pub fn d128(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DecimalParser);
    to_code(input.value, input.scale)
}

fn to_code(value: i128, scale: u32) -> TokenStream {
    quote! {
        D128s12::from_bits(#value)
    }.into()
}

struct DecimalParser {
    value: i128,
    scale: u32,
    radix: Option<u32>,
}

impl Parse for DecimalParser {
    fn parse(input: ParseStream) -> Result<Self> {
        let value: i128 = 0;
        let scaled: u32 = 12;
        let radix: u32 = 10;

        Ok(DecimalParser {
            value: 0,
            scale: 12,
            radix: Some(10),
        })
    }
}
