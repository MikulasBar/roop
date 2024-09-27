use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, Fields, Field};

use crate::get_macro_name;

pub fn class_implementation(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let ItemStruct { 
        ref ident, 
        ref fields, 
        ..
    } = input;

    match fields {
        Fields::Named(fields) => (),
        _ => return quote! {
            compile_error!("class attribute only supports named fields");
        }.into(),
    }

    let macro_name = get_macro_name(ident);
    let fields: Vec<Field> = fields.clone()
        .into_iter()
        .collect();

    let macro_def = quote! {
        macro_rules! #macro_name {
            ($( ##[$meta:meta] )* $vis:vis struct $s:ident { $( $f:ident : $t:ty ),*}) => {
                $( ##[$meta] )*
                $vis struct $s {
                    #(#fields),*,
                    $($f : $t),*
                }
            };

            ($($tt:tt)*) => {
                const _: () = ();
                compile_error!("class extender macro can't parse provided syntax - error inside API");
            };
        }
    };

    quote! {
        #input
        #macro_def
    }.into()
}