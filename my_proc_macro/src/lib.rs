extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemFn};

// attribute-like macro
#[proc_macro_attribute]
pub fn function_to_string(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input function
    let inupt_fn: ItemFn = parse_macro_input!(item as ItemFn);

    // Create a string representation of the function
    let function_str: String = format!("{}", inupt_fn.to_token_stream());

    // Define a new function with the same signature as the input function
    let fn_ident: proc_macro2::Ident = inupt_fn.sig.ident;
    let fn_inputs: syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma> = inupt_fn.sig.inputs;
    let fn_generics: syn::Generics = inupt_fn.sig.generics;

    // Generate the putput code
    let output: proc_macro2:: TokenStream = quote! {
        pub fn #fn_ident #fn_generics(#fn_inputs) -> &'static str {
            #function_str
        }
    };

    output.into()
}


// // 派生マクロとして最初に宣言
// #[proc_macro_derive(HelloWorld)]
// pub fn helloworld_object_derive(input: TokenStream) -> TokenStream {
//     // Parse the input tokens into a syntax tree
//     let input = parse_macro_input!(input as DeriveInput);
//     //  Used in the quasi-quotation below as the name of the type to implement
//     let name = input.ident;
//     // Generate the code to paste into the user's program
//     let expanded = quote! {
//         impl HelloWorld for #name {
//             fn hello_world() {
//                 print1n!("Hello, world!");
//             }
//         }
//     }
//     // Hand the output tokens back to the compiler
//     TokenStream::from(expanded)
// }