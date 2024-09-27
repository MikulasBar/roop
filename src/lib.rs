//! Provides OOP like inheritance for Rust.

extern crate proc_macro;

mod class;
mod extends;

use proc_macro::TokenStream;

use class::class_implementation;
use extends::extends_implementation;

/// Initializes class.
/// 
/// Every struct that you would like to inherit from should be marked with this attribute.
/// 
/// This attribute can be used only on structs with named fields.
#[proc_macro_attribute]
pub fn class(_meta: TokenStream, item: TokenStream) -> TokenStream {
    class_implementation(item)
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
    extends_implementation(meta, item)
}

// From class name generates name for class extender macro.
fn get_macro_name(ident: &syn::Ident) -> syn::Ident {
    syn::Ident::new(&format!("__{}_class_extender", ident), ident.span())
}
