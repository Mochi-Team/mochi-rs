// #![no_std]

mod parser;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn mochi_bind(attr: TokenStream, input: TokenStream) -> TokenStream {
    match parser::expand(attr.into(), input.into()) {
        Ok(tokens) => tokens.into(),
        Err(error) => {
            error.to_compile_error().into()
        }
    }
}