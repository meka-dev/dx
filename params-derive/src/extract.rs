use proc_macro_error::abort_call_site;
use syn::{Attribute, Fields, Generics, Ident, ItemStruct, Visibility};

use crate::{parse::Ast, route::Route, serialize_field::SerializeField, value::Value};

const FIELD_ATTRIBUTE: &str = "value";

pub struct Model {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,

    pub routes: Vec<Route>,
    pub serialize_fields: Vec<SerializeField>,
    pub values: Vec<Value>,
}

pub fn extract(item: Ast) -> Model {
    let ItemStruct {
        mut attrs,
        vis,
        ident,
        generics,
        ..
    } = item;

    // Remove this macros attribute from the item.
    attrs.retain(|attr| {
        if let Some(ident) = attr.path.get_ident() {
            if ident.to_string().as_str() == "params" {
                return false;
            }
        }
        true
    });

    let fields = match item.fields {
        Fields::Named(fields) => fields,
        _ => abort_call_site!("`#[params]` only operates on named fields"; help = "`#[params]`"),
    };

    let values = fields
        .named
        .iter()
        .cloned()
        .map(|mut field| {
            // TODO(xla): Parse actual attribute TokenStream to get paramterisation.
            let dynamic = field.attrs.iter().any(|attr| {
                attr.path.get_ident().map(ToString::to_string) == Some(FIELD_ATTRIBUTE.to_string())
            });
            field.attrs.retain(|attr| {
                let ident = attr.path.get_ident();

                ident.is_none()
                    || ident.map(ToString::to_string) != Some(FIELD_ATTRIBUTE.to_string())
            });

            Value {
                field: field.clone(),
                dynamic,
            }
        })
        .collect::<Vec<_>>();
    let serialize_fields = values
        .iter()
        .map(|val| SerializeField {
            ident: val.field.ident.clone().unwrap(),
            dynamic: val.dynamic,
        })
        .collect::<Vec<_>>();
    let routes = values
        .iter()
        .filter_map(|val| {
            if !val.dynamic {
                return None;
            }

            Some(Route {
                ident: val.field.ident.clone().unwrap(),
                ty: val.field.ty.clone(),
            })
        })
        .collect::<Vec<_>>();

    Model {
        attrs,
        vis,
        ident,
        generics,

        routes,
        serialize_fields,
        values,
    }
}
