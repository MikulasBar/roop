extern crate proc_macro;

use quote::quote;
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemStruct, Path, Field};

fn get_macro_name(ident: &syn::Ident) -> syn::Ident {
    syn::Ident::new(&format!("__{}_class_extender", ident), ident.span())
}


#[proc_macro_attribute]
pub fn class(_meta: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let ItemStruct { 
        ref ident, 
        ref fields, 
        ..
    } = input;

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
                panic!("{:?} can't parse provided syntax: {:?}", #macro_name, stringify!($($tt)*));
            };
        }
    };

    quote! {
        #input
        #macro_def
    }.into()
}





#[proc_macro_attribute]
pub fn extends(meta: TokenStream, item: TokenStream) -> TokenStream {
    let parent = parse_macro_input!(meta as Path);
    let child = parse_macro_input!(item as ItemStruct);

    let ItemStruct { 
        attrs, 
        vis, 
        ident, 
        generics, 
        fields, 
        semi_token,
        ..
    } = child;

    let parent_ident = parent.segments.last().unwrap().ident.clone();
    let parent_macro = get_macro_name(&parent_ident);
    let fields: Vec<Field> = fields.into_iter().collect();

    let struct_def = quote! {
        #parent_macro!{ 
            #(#attrs)*
            #vis struct #ident {
                #(#fields),*
            }
            
        }
        #semi_token
    };

    // panic!("child_struct_def: {}", struct_def);
    
    let deref_traits = quote! {
        impl std::ops::Deref for #ident {
            type Target = #parent_ident;
            
            fn deref(&self) -> &Self::Target {
                unsafe { &*(self as *const #ident as *const #parent) }
            }
        }
        
        impl std::ops::DerefMut for #ident {
            fn deref_mut(&mut self) -> &mut Self::Target {
                unsafe { &mut *(self as *mut #ident as *mut #parent) }
            }
        }
    };
    
    quote! {
        #struct_def
        #deref_traits
    }.into()
}
