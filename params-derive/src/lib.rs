#![warn(clippy::nursery, clippy::pedantic)]

use proc_macro_error::proc_macro_error;

use codegen::codegen;
use extract::extract;
use parse::parse;

mod codegen;
mod extract;
mod parse;
mod route;
mod serialize_field;
mod value;

#[proc_macro_attribute]
#[proc_macro_error]
pub fn params(
    args: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ast = parse(args.into(), item.into());
    let model = extract(ast);
    let rust = codegen(model);

    rust.into()
}
