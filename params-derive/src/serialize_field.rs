use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Ident;

pub struct SerializeField {
    pub ident: Ident,
    pub dynamic: bool,
}

impl ToTokens for SerializeField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let name = ident.to_string();
        let dynamic = self.dynamic;

        let val = if self.dynamic {
            quote! {
                &(*self.#ident.blocking_read())
            }
        } else {
            quote! {
                &self.#ident
            }
        };

        let out = quote! {
            s.serialize_field(#name, &SerializeField { dynamic: #dynamic, value: #val })?;
        };
        out.to_tokens(tokens);
    }
}
