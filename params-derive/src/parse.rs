#![allow(clippy::option_if_let_else)]

use proc_macro2::TokenStream;
use proc_macro_error::{abort, abort_call_site};
use syn::{Expr, Item};

pub type Ast = syn::ItemStruct;

pub fn parse(args: TokenStream, item: TokenStream) -> Ast {
    const ERROR: &str = "this attribute takes no arguments";
    const HELP: &str = "use `#[params]`";

    if !args.is_empty() {
        if let Ok(expr) = syn::parse2::<Expr>(args) {
            abort!(expr, ERROR; help = HELP)
        } else {
            abort_call_site!(ERROR; help = HELP)
        }
    }

    match syn::parse2::<Item>(item) {
        Ok(Item::Struct(item)) => item,
        Ok(item) => {
            abort!(item, "item is not a struct"; help = "`#[params]` can only be used on structs")
        },
        Err(_) => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::parse;

    #[test]
    fn valid_syntax() {
        parse(
            quote!(),
            quote!(
                #[params]
                struct Params {
                    pub readable: String,

                    #[writable]
                    pub toggle: bool,
                }
            ),
        );
    }
}
