#[cfg(not(any(feature = "frontend", feature = "backend")))]
compile_error!("one of features \"frontend\" or \"backend\" must be enabled");

#[cfg(all(feature = "frontend", feature = "backend"))]
compile_error!("features \"frontend\" and \"backend\" cannot be enabled at the same time");

use proc_macro::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, FnArg};

#[proc_macro_attribute]
pub fn command(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let vis = &input.vis;
    let asyncness = &input.sig.asyncness;
    let ret = &input.sig.output;
    let name = &input.sig.ident;
    let inputs = &input.sig.inputs;
    let generics = &input.sig.generics;
    let body = &input.block;
    let attrs = &input.attrs;
    let name_str = name.to_string();
    let input_names = inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Typed(pat) => Some(*pat.pat.clone()),
            _ => None,
        })
        .collect::<Punctuated<_, Comma>>();
    let struct_name = quote::format_ident!("__command__{}", name);

    let result_type = quote! {
        #[allow(non_camel_case_types)]
        #[derive(Debug, ::serde::Serialize, ::serde::Deserialize)]
        pub struct #struct_name {
            #inputs
        }
    };

    let result_fn = if cfg!(feature = "frontend") {
        quote! {
            #(#attrs)*
            #vis async fn #name #generics(#inputs) #ret {
                let args = #struct_name {
                    #input_names
                };
                let res = ::macro_utils::tauri_command(#name_str, &args).await;
                res
            }
        }
    } else if cfg!(feature = "backend") {
        quote! {
            #[::macro_utils::tauri::command(async)]
            #(#attrs)*
            #vis #asyncness fn #name #generics(args: #struct_name) #ret {
                let #struct_name { #input_names } = args;
                #body
            }
        }
    } else {
        unreachable!()
    };

    quote! {
        #result_type

        #result_fn
    }
    .into()
}
