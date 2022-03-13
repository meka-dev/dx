use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Field;

pub struct Value {
    pub field: Field,
    pub dynamic: bool,
}

impl ToTokens for Value {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Value {
            field:
                Field {
                    attrs,
                    vis,
                    ident,
                    ty,
                    ..
                },
            dynamic,
        } = self;

        let wrapped_ty = if *dynamic {
            quote! { ::std::sync::Arc<::std::sync::RwLock<#ty>> }
        } else {
            quote! { #ty }
        };
        let out = quote! {
            #(#attrs)*
            #vis #ident: #wrapped_ty
        };
        out.to_tokens(tokens);
    }
}
