extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;


#[proc_macro_derive(Vaapad)]
pub fn vaapad(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();
    // Build the impl
    let gen = impl_vaapad(&ast);
    // Return the generated impl
    gen.parse().unwrap()
}



fn impl_vaapad(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl Vaapad for #name {
            fn get_type_id() {
                let fullname = concat!(module_path!(), "::", stringify!($expr))

            }
        }
    }
}