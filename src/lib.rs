extern crate proc_macro;

mod class;
mod extends;
use proc_macro::TokenStream;

use class::class_implementation;
use extends::extends_implementation;


#[proc_macro_attribute]
pub fn class(_meta: TokenStream, item: TokenStream) -> TokenStream {
    class_implementation(item)
}

#[proc_macro_attribute]
pub fn extends(meta: TokenStream, item: TokenStream) -> TokenStream {
    extends_implementation(meta, item)
}

fn get_macro_name(ident: &syn::Ident) -> syn::Ident {
    syn::Ident::new(&format!("__{}_class_extender", ident), ident.span())
}
