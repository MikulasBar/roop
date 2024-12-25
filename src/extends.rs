use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, Fields, Field, Path};

use crate::get_ce_name;

pub fn extends_implementation(meta: TokenStream, item: TokenStream) -> TokenStream {
    let parent = parse_macro_input!(meta as Path);
    let child = parse_macro_input!(item as ItemStruct);

    let ItemStruct { 
        attrs, 
        vis, 
        ident, 
        fields, 
        semi_token,
        ..
    } = child;

    // If the struct is tuple or unit, return an error.
    match fields {
        Fields::Named(_) => (),
        _ => return quote! {
            compile_error!("extends attribute only supports named fields");
        }.into(),
    }

    // Get only the last segment of the parent path, the type name.
    // This isn't problem because all classes are in the root of library so the path shouldn't be used anyway. 
    let parent_ident = parent.segments.last().unwrap().ident.clone();
    let parent_ce = get_ce_name(&parent_ident);
    // Same as in class, we need to convert fields to Vec to avoid additional braces.
    let fields: Vec<Field> = fields.into_iter().collect();
    // pure path to location of parent
    let parent_only_path = get_only_path(parent.clone());
    // We use the class extender macro to insert parent fields into our struct.
    // crate:: is there because every class class extender macro is exported to the root of library
    let struct_def = quote! {
        #parent_only_path::#parent_ce!{ 
            #(#attrs)*
            #vis struct #ident {
                #(#fields),*
            }
        }
        #semi_token
    };

    // Implement Deref and DerefMut for the child struct.
    // This will allow us to implicitly convert the child struct to the parent struct whenever we need to use methods from parent.
    // We implement this by converting child pointer to parent pointer.
    // Because of this, we need to make sure that the fields from parent is the first in the struct.
    // Otherwise, the pointer conversion will not work correctly.
    // This is ensured in class_implementation function.
    // So although this needs unsafe code, it is safe.
    let deref_traits = quote! {
        impl std::ops::Deref for #ident {
            type Target = #parent;
            
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

// get only the path but not the item at the end
fn get_only_path(mut path: Path) -> Path {
    let len = path.segments.len();
    if len == 1 {
        return path;
    }
    
    path.segments = path.segments.into_iter()
        .take(len - 1)
        .collect();

    path
}