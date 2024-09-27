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

    // If the struct is tuple or unit, return an error.
    match fields {
        Fields::Named(_) => (),
        _ => return quote! {
            compile_error!("class attribute only supports named fields");
        }.into(),
    }

    let macro_name = get_macro_name(ident);
    // If we don't convert the fields to a Vec, the interpolation of the fields will generate additional braces around it.
    let fields: Vec<Field> = fields.clone()
        .into_iter()
        .collect();

    // Generates class extender macro.
    // This macro will take provided struct definition and will insert additional fields from parent class.
    // This is how we can later get access to the fields of the parent class from other parts of code.
    // Note that the macro has to take the entire struct definition, because declarative macros can't expand into fields.
    // ## is used in quote macro to escape the #.
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
                const _: () = (); // The constant serves as placeholder item for attributes, because attributes can be used only on items. 
                compile_error!("class extender macro can't parse provided syntax - error inside the attribute");
            };
        }
    };

    // Returns the struct definition and the class extender macro.
    quote! {
        #input
        #macro_def
    }.into()
}