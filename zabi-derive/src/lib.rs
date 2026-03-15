extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::LitStr;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(ZDecode)]
pub fn zabi_decode_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let struct_name = name.clone();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let decode_body = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => {
                let field_recurse = fields.named.iter().map(|f| {
                    let field_name = f.ident.as_ref().expect("named field");
                    let ty = &f.ty;
                    quote! {
                        #field_name: {
                            let field_offset = offset;
                            let val = <#ty as ::zabi_rs::ZDecode>::decode(data, field_offset)
                                .map_err(|err| err.with_context(concat!(stringify!(#struct_name), ".", stringify!(#field_name)), field_offset))?;
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
                        if offset > data.len() {
                            return Err(::zabi_rs::ZError::OutOfBounds(offset, data.len()));
                        }
                        let data = &data[offset..];
                        let mut offset: usize = 0;
                        Ok(#name {
                            #(#field_recurse),*
                        })
                    }
                }
            }
            Fields::Unnamed(fields) => {
                let field_recurse = fields.unnamed.iter().enumerate().map(|(index, f)| {
                    let ty = &f.ty;
                    let field_label = LitStr::new(&format!("{}.{}", struct_name, index), Span::call_site());
                    quote! {
                        {
                            let field_offset = offset;
                            let val = <#ty as ::zabi_rs::ZDecode>::decode(data, field_offset)
                                .map_err(|err| err.with_context(#field_label, field_offset))?;
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
                        if offset > data.len() {
                            return Err(::zabi_rs::ZError::OutOfBounds(offset, data.len()));
                        }
                        let data = &data[offset..];
                        let mut offset: usize = 0;
                        Ok(#name (
                            #(#field_recurse),*
                        ))
                    }
                }
            }
            Fields::Unit => {
                quote! {
                    const HEAD_SIZE: usize = 0;
                    fn decode(data: &'a [u8], offset: usize) -> Result<Self, ::zabi_rs::ZError> {
                        if offset > data.len() {
                            return Err(::zabi_rs::ZError::OutOfBounds(offset, data.len()));
                        }
                        Ok(#name)
                    }
                }
            }
        },
        _ => panic!("ZDecode can only be derived for structs"),
    };

    let expanded = quote! {
        impl #impl_generics ::zabi_rs::ZDecode<'a> for #name #ty_generics #where_clause {
            #decode_body
        }
    };

    TokenStream::from(expanded)
}
