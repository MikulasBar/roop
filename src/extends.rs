use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, Fields, Field, Path};

use crate::get_macro_name;

pub fn extends_implementation(meta: TokenStream, item: TokenStream) -> TokenStream {
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

    match fields {
        Fields::Named(ref fields) => (),
        _ => return quote! {
            compile_error!("extends attribute only supports named fields");
        }.into(),
    }

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