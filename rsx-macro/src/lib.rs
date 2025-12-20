use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let attrs = input.attrs;
    let vis = input.vis;
    let sig = input.sig;
    let block = input.block;

    let output = quote! {
        #(#attrs)*
        #[allow(non_snake_case)]
        #vis #sig #block
    };

    TokenStream::from(output)
}
