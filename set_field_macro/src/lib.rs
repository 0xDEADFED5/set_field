//! derive macro for set_field
use proc_macro::TokenStream;
use quote::quote;
use std::collections::BTreeMap;
use syn::{Data, DeriveInput, Fields, Ident, Type};

#[proc_macro_derive(SetField)]
/// # Example
/// ```
/// use set_field::SetField;
/// #[derive(SetField)]
/// struct Foo { a: i32 }
/// fn test() {
/// 	let mut foo = Foo { a: 777 };
/// 	foo.set_field("a", 888);
/// }
// ```
pub fn set_field_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_set_field(&ast)
}

fn impl_set_field(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    // indirect BTreeMap<syn::Type, Vec<Ident>>
    let mut type_map: BTreeMap<usize, Vec<Ident>> = BTreeMap::new();
    let mut types: Vec<Type> = vec![];
    match &ast.data {
        Data::Struct(ref data) => match &data.fields {
            Fields::Named(fields) => {
                for f in &fields.named {
                    match &f.ident {
                        Some(i) => {
                            if !types.contains(&f.ty) {
                                types.push(f.ty.to_owned());
                            }
                            let index = types.iter().position(|x| x == &f.ty).unwrap();
                            match type_map.get_mut(&index) {
                                Some(v) => {
                                    v.push(i.clone());
                                }
                                None => {
                                    type_map.insert(index, vec![i.clone()]);
                                }
                            }
                        }
                        None => {}
                    }
                }
            }
            _ => {}
        },
        _ => {
            panic!("struct only!");
        }
    };
    let mut impls = quote!();
    let mut matches;
    for x in type_map {
        matches = quote!();
        let ty = &types[x.0];
        for id in x.1 {
            let s = id.to_string();
            matches.extend(quote! {
                #s => {
                    self.#id = value;
                    true
                }
            });
        }
        impls.extend(quote! {
            impl SetField<#ty> for #name {
                fn set_field(&mut self, field: &str, value: #ty) -> bool {
                    match field {
                        #matches
                        _ => { false }
                    }
                }
            }
        });
    }
    impls.into()
}