//! Provides OOP like inheritance for Rust.

extern crate proc_macro;

mod class;
mod extends;
mod import;

use proc_macro::TokenStream;

/// Initializes class.
/// 
/// Every struct that you would like to extend, should be marked with this attribute.
/// 
/// This can be used only on structs with named fields.
#[proc_macro_attribute]
pub fn class(_meta: TokenStream, item: TokenStream) -> TokenStream {
    class::class_implementation(item)
}

/// Extends class.
/// 
/// Every struct that you would like to extend from another struct should be marked with this attribute.
/// 
/// The struct that you extend don't need to be marked with `class` attribute.
/// 
/// This attribute can be used only on structs with named fields.
#[proc_macro_attribute]
pub fn extends(meta: TokenStream, item: TokenStream) -> TokenStream {
    extends::extends_implementation(meta, item)
}

#[proc_macro]
pub fn import(input: TokenStream) -> TokenStream {
    import::import_implementation(input)
}

// From class name generates name for class extender macro.
fn get_ce_name(ident: &syn::Ident) -> syn::Ident {
    syn::Ident::new(&format!("{}{}", CE_PREFIX, ident), ident.span())
}

// class extender prefix
const CE_PREFIX: &str = "__roop_ce_";