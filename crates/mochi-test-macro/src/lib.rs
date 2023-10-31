extern crate proc_macro2;
extern crate quote;
extern crate syn;

mod parser;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn mochi_test(
    _attr: TokenStream, 
    input: TokenStream
) -> TokenStream {
    input.into()
}