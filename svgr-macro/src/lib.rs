extern crate proc_macro2;

mod node;
mod nodes_to_format;
mod parser;
mod validate_svg;

use proc_macro::TokenStream;
use quote::quote;

use syn::{
    parse::{ParseStream, Parser as _},
    Result,
};

use node::Node;
use parser::{Parser, ParserOptions};

use crate::nodes_to_format::prepare_svg_nodes_for_format_statement;

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
            let (html_string, values, animations) = prepare_svg_nodes_for_format_statement(nodes);

            quote! {
                {
                lazy_static::lazy_static! {
                    // static ref COUNT: usize = 12;
                    #(#animations)*
                }

                format!(#html_string, #(#values),*)
            }
            }
        }
        Err(error) => error.to_compile_error(),
    }
    .into()
}
