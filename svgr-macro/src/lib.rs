extern crate proc_macro;

mod node;
mod parser;
mod nodes_to_format;
mod validate_svg;

use proc_macro::TokenStream;
use quote::quote;

use syn::{
    parse::{ParseStream, Parser as _},
    Result,
};

use node::Node;
use parser::{Parser, ParserOptions};

use crate::nodes_to_format::nodes_to_format;

mod punctuation {
    use syn::custom_punctuation;

    custom_punctuation!(Dash, -);
}

fn parse(tokens: proc_macro::TokenStream) -> Result<Vec<Node>> {
    let parser = move |input: ParseStream| Parser::new(ParserOptions::default()).parse(input);

    parser.parse(tokens)
}

fn parse_with_config(tokens: proc_macro::TokenStream, config: ParserOptions) -> Result<Vec<Node>> {
    let parser = move |input: ParseStream| Parser::new(config).parse(input);

    parser.parse(tokens)
}

#[proc_macro]
pub fn svgr(tokens: TokenStream) -> TokenStream {
    match parse(tokens) {
        Ok(nodes) => {
            let (html_string, values) = nodes_to_format(nodes);
            quote! { format!(#html_string, #(#values),*) }
        }
        Err(error) => error.to_compile_error(),
    }
    .into()
}
