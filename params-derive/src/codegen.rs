use proc_macro2::TokenStream;
use quote::quote;

use crate::extract::Model;

pub fn codegen(model: Model) -> TokenStream {
    let Model {
        attrs,
        vis,
        ident,
        generics,

        routes,
        serialize_fields,
        values,
    } = model;
    let name = ident.to_string();
    let len = values.len();
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        #[derive(Clone)]
        #(#attrs)*
        #vis struct #ident #generics #where_clause {
            #(#values),*
        }

        use serde::ser::SerializeStruct as _;
        impl #impl_generics serde::Serialize for #ident #ty_generics #where_clause {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::ser::Serializer,
            {
                #[derive(serde::Serialize)]
                struct SerializeField<T> {
                    dynamic: bool,
                    value: T,
                }
                let mut s = serializer.serialize_struct(#name, #len)?;
                #(#serialize_fields)*
                s.end()
            }
        }

        impl #impl_generics #ident #ty_generics #where_clause {
            pub fn router(self) -> (#ident #ty_generics, axum::Router) {
                use std::sync::Arc;

                let params = Arc::new(self.clone());

                let router = axum::Router::new()
                    #(#routes)*
                    .route(
                        "/",
                        axum::routing::get({
                            let params = Arc::clone(&params);
                            move || Self::get_params(Arc::clone(&params))
                        }),
                    );

                (self, router)
            }

            async fn get_params(
                params: std::sync::Arc<#ident #ty_generics>,
            ) -> axum::http::Response<axum::body::Body> {
                let body = serde_json::to_vec(&params).unwrap();
                axum::http::Response::builder().body(body.into()).unwrap()
            }

            async fn put_param<V>(
                axum::Json(value): axum::Json<V>,
                param: std::sync::Arc<std::sync::RwLock<V>>,
            ) -> impl axum::response::IntoResponse {
                let mut param = param.write().unwrap();
                *param = value;

                axum::http::StatusCode::NO_CONTENT
            }
        }
    }
}
