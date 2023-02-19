use commands::FRONTENDCOMMANDS_METHODS;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, Fields, FnArg, ItemFn, ItemStruct,
    Pat, Signature, Type, Visibility,
};

#[proc_macro_derive(FrontendCommands)]
pub fn derive_frontend_commands(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    let ident = &input.ident;

    let mut methods = Vec::new();
    let mut method_arg_structs = Vec::new();

    for method_str in FRONTENDCOMMANDS_METHODS {
        let method_tokens = method_str.parse::<TokenStream>().unwrap();
        let method = parse_macro_input!(method_tokens as Signature);
        let method_name = &method.ident;
        let method_name_str = method_name.to_string();
        let struct_name = quote::format_ident!("__command__{}", method_name);
        let inputs = method
            .inputs
            .iter()
            .filter(|arg| match arg {
                FnArg::Receiver(_) => false,
                FnArg::Typed(_) => true,
            })
            .collect::<Punctuated<_, Comma>>();
        let input_names = inputs
            .iter()
            .filter_map(|arg| match arg {
                FnArg::Typed(pat) => Some(*pat.pat.clone()),
                _ => None,
            })
            .collect::<Punctuated<_, Comma>>();

        method_arg_structs.push(quote! {
            #[allow(non_camel_case_types)]
            #[derive(Debug, ::serde::Serialize, ::serde::Deserialize)]
            struct #struct_name {
                #inputs
            }
        });

        methods.push(quote! {
            #method {
                let args = #struct_name {
                    #input_names
                };
                let res = tauri_command(#method_name_str, &args).await;
                res
            }
        });
    }

    quote! {
        #(#method_arg_structs)*

        #[::async_trait::async_trait(?Send)]
        impl ::commands::FrontendCommands for #ident {
            #(#methods)*
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn props(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    quote! {
        #[derive(::yew::Properties, PartialEq, Clone)]
        #input
    }
    .into()
}

#[proc_macro_attribute]
pub fn async_props(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemStruct);
    let test_fields = quote! {
        struct Test {
            #[prop_or(::yew::virtual_dom::VNode::default())]
            pub fallback: ::yew::virtual_dom::VNode,
        }
    }
    .into();
    let test_fields_input = parse_macro_input!(test_fields as ItemStruct);
    let test_fields_fallback = test_fields_input.fields.iter().next().unwrap();
    match &mut input.fields {
        Fields::Named(named_fields) => named_fields.named.push(test_fields_fallback.to_owned()),
        _ => {
            return syn::Error::new(
                input.ident.span(),
                "expected named fields in async properties struct",
            )
            .to_compile_error()
            .into()
        }
    }

    quote! {
        #[derive(::yew::Properties, PartialEq, Clone)]
        #input
    }
    .into()
}

#[proc_macro_attribute]
pub fn async_function_component(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let body = &*input.block;
    let inner_ident = format_ident!("{}Inner", input.sig.ident);
    let mut params = input.sig.inputs.iter().collect::<Vec<_>>();
    let state_param_index = params.iter().position(|&param| match param {
        FnArg::Typed(pat) => match &*pat.ty {
            Type::Path(type_path) => {
                type_path.path.segments.last().unwrap().ident.to_string() == "StoreState"
            }
            _ => false,
        },
        FnArg::Receiver(_) => false,
    });
    let state_param = state_param_index.map(|index| params.remove(index));
    let props_param_arg = params.first().map(|x| x.to_owned());
    let props_param = params
        .first()
        .map(|param| match param {
            FnArg::Typed(pat) => Some(&*pat.pat),
            FnArg::Receiver(_) => None,
        })
        .flatten();
    let props_param_ident = props_param
        .clone()
        .map(|props| match props {
            Pat::Ident(pat_ident) => pat_ident.ident.clone(),
            _ => unreachable!(),
        })
        .unwrap_or(format_ident!("props"));

    let return_test = quote! { fn test() -> ::yew::HtmlResult {} }.into();
    let return_test_input = parse_macro_input!(return_test as ItemFn);

    let inner_state_usage = match state_param {
        Some(param) => match param {
            FnArg::Typed(pat) => {
                quote! {
                    let #pat = ::yewdux::prelude::use_store::<crate::state::State>();
                }
            }
            FnArg::Receiver(_) => unreachable!(),
        },
        None => quote! {},
    };

    let inner_body_test = quote! {
        fn test() {
            #inner_state_usage

            let #props_param_ident = #props_param_ident.clone();

            let res = ::yew::suspense::use_future(|| async move {
                let #props_param_ident = &#props_param_ident.clone();

                #body
            })?;

            Ok((*res).to_owned())
        }
    }
    .into();
    let inner_body_test_input = parse_macro_input!(inner_body_test as ItemFn);

    let (component_params, params_struct) = {
        let (params_test, params_struct) = match props_param_arg {
            Some(param) => (
                quote! {
                    fn test(#param) {}
                },
                quote! {},
            ),
            None => (
                quote! {
                    fn test(props: &Props) {}
                },
                quote! {
                    #[derive(::yew::prelude::Properties, PartialEq, Clone)]
                    pub struct Props {
                        #[prop_or(::yew::virtual_dom::VNode::default())]
                        pub fallback: ::yew::virtual_dom::VNode,
                    }
                },
            ),
        };
        let params_test_into = params_test.into();
        let params_test_input = parse_macro_input!(params_test_into as ItemFn);
        (params_test_input.sig.inputs, params_struct)
    };

    let outer_body_inner_call = match props_param {
        Some(param) => quote! { <#inner_ident ..#param.clone()></#inner_ident> },
        None => quote! { <#inner_ident></#inner_ident> },
    };
    let outer_body_test = quote! {
        fn test() {
            let fallback = #props_param_ident.fallback.to_owned();

            html! {
                <::yew::Suspense {fallback}>
                    #outer_body_inner_call
                </::yew::Suspense>
            }
        }
    }
    .into();
    let outer_body_test_input = parse_macro_input!(outer_body_test as ItemFn);

    let mut inner_component = input.clone();
    inner_component.vis = Visibility::Inherited;
    inner_component.sig.asyncness = None;
    inner_component.sig.ident = inner_ident;
    inner_component.sig.inputs = component_params.to_owned();
    inner_component.sig.output = return_test_input.sig.output;
    inner_component.block = inner_body_test_input.block;

    let mut outer_component = input.clone();
    outer_component.sig.asyncness = None;
    outer_component.sig.inputs = component_params.to_owned();
    outer_component.block = outer_body_test_input.block;

    quote! {
        #params_struct

        #[::yew::function_component]
        #inner_component

        #[::yew::function_component]
        #outer_component
    }
    .into()
}
