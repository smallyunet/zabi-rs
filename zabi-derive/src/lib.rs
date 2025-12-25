extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(ZDecode)]
pub fn zabi_decode_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let decode_body = match input.data {
        Data::Struct(data) => {
            match data.fields {
                Fields::Named(fields) => {
                    let field_recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        let ty = &f.ty;
                        quote! {
                            #name: {
                                let val = <#ty as ::zabi_rs::ZDecode>::decode(data, offset)?;
                                offset += <#ty as ::zabi_rs::ZDecode>::HEAD_SIZE;
                                val
                            }
                        }
                    });
                    let head_size_recurse = fields.named.iter().map(|f| {
                        let ty = &f.ty;
                        quote! { <#ty as ::zabi_rs::ZDecode>::HEAD_SIZE }
                    });
                    
                    quote! {
                        const HEAD_SIZE: usize = 0 #(+ #head_size_recurse)*;
                        fn decode(data: &'a [u8], offset: usize) -> Result<Self, ::zabi_rs::ZError> {
                            let mut offset = offset;
                            Ok(#name {
                                #(#field_recurse),*
                            })
                        }
                    }
                }
                Fields::Unnamed(fields) => {
                    let field_recurse = fields.unnamed.iter().map(|f| {
                        let ty = &f.ty;
                        quote! {
                            {
                                let val = <#ty as ::zabi_rs::ZDecode>::decode(data, offset)?;
                                offset += <#ty as ::zabi_rs::ZDecode>::HEAD_SIZE;
                                val
                            }
                        }
                    });
                    let head_size_recurse = fields.unnamed.iter().map(|f| {
                        let ty = &f.ty;
                        quote! { <#ty as ::zabi_rs::ZDecode>::HEAD_SIZE }
                    });
                    quote! {
                        const HEAD_SIZE: usize = 0 #(+ #head_size_recurse)*;
                        fn decode(data: &'a [u8], offset: usize) -> Result<Self, ::zabi_rs::ZError> {
                            let mut offset = offset;
                            Ok(#name (
                                #(#field_recurse),*
                            ))
                        }
                    }
                }
                Fields::Unit => {
                    quote! { 
                        const HEAD_SIZE: usize = 0;
                        fn decode(data: &'a [u8], _offset: usize) -> Result<Self, ::zabi_rs::ZError> {
                            Ok(#name)
                        }
                    }
                }
            }
        }
        _ => panic!("ZDecode can only be derived for structs"),
    };

    let expanded = quote! {
        impl #impl_generics ::zabi_rs::ZDecode<'a> for #name #ty_generics #where_clause {
            #decode_body
        }
    };

    TokenStream::from(expanded)
}
