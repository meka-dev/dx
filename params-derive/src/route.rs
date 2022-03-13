use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Ident, Type};

pub struct Route {
    pub ident: Ident,
    pub ty: Type,
}

impl ToTokens for Route {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let ty = &self.ty;
        let route = format!("/{}", self.ident);

        let out = quote! {
            .route(#route, axum::routing::put({
                let param = Arc::clone(&self.#ident);
                move |body| Self::put_param::<#ty>(body, param)
            }))
        };

        out.to_tokens(tokens);
    }
}
