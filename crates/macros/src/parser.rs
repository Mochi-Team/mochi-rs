extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_quote, punctuated::Punctuated, spanned::Spanned, Error, Result};

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
            syn::Item::Fn(f) => {
                let func_name = &f.sig.ident;

                let wasn_func_name = format_ident!("__wasm_{}", func_name.clone());
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
                                syn::Type::Reference(syn::TypeReference {
                                    mutability: Some(_),
                                    elem,
                                    ..
                                }) => {
                                    wasm_func_args.push(parse_quote! {
                                        #ident: <#elem as mochi::convert::RefMutFromWasm>::Value
                                    });
                                    wasm_func_variables.push(quote! {
                                        let mut #ident = unsafe {
                                            <#elem as mochi::convert::RefMutFromWasm>
                                                ::ref_mut_from_wasm(#ident)
                                        };
                                        let #ident = &mut *#ident;
                                    });
                                },
                                syn::Type::Reference(syn::TypeReference { elem, .. }) => {
                                    wasm_func_args.push(parse_quote! {
                                        #ident: <#elem as mochi::convert::RefFromWasm>::Value
                                    });
                                    wasm_func_variables.push(quote! {
                                        let #ident = unsafe {
                                            <#elem as mochi::convert::RefFromWasm>
                                                ::ref_from_wasm(#ident)
                                        };
                                        let #ident = &*#ident;
                                    });            
                                },
                                _ => {
                                    wasm_func_args.push(parse_quote! {
                                        #ident: <#ty as mochi::convert::FromWasm>::Value
                                    });
                                    wasm_func_variables.push(quote! {
                                        let #ident = <#ty as mochi::convert::FromWasm>::from_wasm(#ident);
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
                            #func_name(#(#wasm_func_calls,)*)
                        },
                        syn::ReturnType::Default,
                    ),
                    syn::ReturnType::Type(_, ty) => (
                        quote! {
                            let ret_val = #func_name(#(#wasm_func_calls,)*);
                            <#ty as mochi::convert::ToWasm>::to_wasm(ret_val)
                        },
                        parse_quote! {
                            -> <#ty as mochi::convert::ToWasm>::Value
                        },
                    ),
                };

                tokens.extend(quote! {
                    #[no_mangle]
                    #[export_name = #wasm_export_name]
                    extern "C" fn #wasn_func_name(#wasm_func_args) #wasm_ret_val {
                        #(#wasm_func_variables;)*
                        #wasm_func_call_block
                    }
                });
                f.to_tokens(tokens)
            }
            _ => {
                return Err(Error::new(self.span(), "cannot use [mochi_bind] on non-func types"));
            }
        }

        Ok(())
    }
}
