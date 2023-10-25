extern crate proc_macro2;
extern crate quote;
extern crate syn;
extern crate alloc;

use alloc::string::ToString;
use alloc::{vec, format};
use alloc::vec::Vec;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_quote, punctuated::Punctuated, spanned::Spanned, Error, Result, ImplItem};

pub fn expand(_: TokenStream, input: TokenStream) -> Result<TokenStream> {
    let item = syn::parse2::<syn::Item>(input)?;
    let mut tokens = TokenStream::new();
    item.macro_parse(&mut tokens)?;
    Ok(tokens)
}

trait MacroParse<Ctx> {
    fn macro_parse(self, context: Ctx) -> Result<()>;
}

impl<'a> MacroParse<&'a mut TokenStream> for syn::Item {
    fn macro_parse(self, tokens: &'a mut TokenStream) -> Result<()> {
        match self {
            syn::Item::Impl(i) => {
                let functions_used: Result<Vec<TokenStream>> = i.items
                    .iter()
                    .map(|o| {
                        parse_impl_item(o)
                    })
                    .collect();

                let type_name = &i.self_ty;

                match functions_used {
                    Ok(functions) => {
                        tokens.extend(quote! {
                            impl #type_name {
                                #(#functions)*
                             }
                        });
        
                        i.to_tokens(tokens);
                        Ok(())
                    },
                    Err(err) => {
                        Err(err)
                    },
                }
            }
            _ => {
                Err(Error::new(self.span(), "[mochi_bind] can only be used on `Meta`, `Video`, `Image`, and `Text` implementations."))
            }
        }
    }
}

fn parse_impl_item(item: &ImplItem) -> Result<TokenStream> {
    match item {
        ImplItem::Fn(f) => {
            let func_name = &f.sig.ident;

            let wasm_func_name = format_ident!("__wasm_{}", func_name.clone());
            let wasm_export_name = format!("{}", func_name.to_string());

            let mut wasm_func_args = Punctuated::<syn::FnArg, syn::token::Comma>::new();
            let mut wasm_func_variables = vec![];
            let mut wasm_func_calls = vec![];

            for (i, arg) in f.sig.inputs.iter().enumerate() {
                let ident = syn::Ident::new(&format!("arg{}", i), proc_macro2::Span::call_site());

                match arg {
                    syn::FnArg::Typed(syn::PatType { ty, .. }) => {
                        wasm_func_calls.push(quote! { #ident });        
                        match &**ty {
                            _ => {
                                wasm_func_args.push(parse_quote! {
                                    #ident: i32
                                });
                                wasm_func_variables.push(quote! {
                                    let #ident: #ty = mochi::std::PtrRef::new(#ident).into();
                                });
                            }
                        }
                    }
                    _ => todo!(),
                }
            }

            let (wasm_func_call_block, wasm_ret_val) = match &f.sig.output {
                syn::ReturnType::Default => (
                    quote! {
                        Self::#func_name(#(#wasm_func_calls,)*)
                    },
                    syn::ReturnType::Default,
                ),
                syn::ReturnType::Type(_, ty) => (
                    quote! {
                        let ret_val: #ty = Self::#func_name(#(#wasm_func_calls,)*);
                        let ptr_ref = mochi::std::PtrRef::from(ret_val);
                        let ret_ptr = ptr_ref.pointer();
                        core::mem::forget(ptr_ref);
                        ret_ptr
                    },
                    parse_quote! {
                        -> i32
                    },
                ),
            };

            Ok(
                quote! {
                    #[no_mangle]
                    #[export_name = #wasm_export_name]
                    extern "C" fn #wasm_func_name(#wasm_func_args) #wasm_ret_val {
                        #(#wasm_func_variables;)*
                        #wasm_func_call_block
                    }
                }
            )
        },
        _ => {
            return Err(Error::new(item.span(), "cannot use [mochi_bind] on non-func types"));
        },
    }
}