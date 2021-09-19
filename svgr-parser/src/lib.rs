extern crate proc_macro;
use syn::{
    parse::{ParseStream, Parser as _},
    Result,
};

mod node;
mod parser;

mod punctuation {
    use syn::custom_punctuation;

    custom_punctuation!(Dash, -);
}

pub use node::{Node, NodeName, NodeType};
pub use parser::{Parser, ParserConfig};

pub fn parse(tokens: proc_macro::TokenStream) -> Result<Vec<Node>> {
    let parser = move |input: ParseStream| Parser::new(ParserConfig::default()).parse(input);

    parser.parse(tokens)
}

pub fn parse_with_config(
    tokens: proc_macro::TokenStream,
    config: ParserConfig,
) -> Result<Vec<Node>> {
    let parser = move |input: ParseStream| Parser::new(config).parse(input);

    parser.parse(tokens)
}

pub fn parse2(tokens: proc_macro2::TokenStream) -> Result<Vec<Node>> {
    let parser = move |input: ParseStream| Parser::new(ParserConfig::default()).parse(input);

    parser.parse2(tokens)
}

pub fn parse2_with_config(
    tokens: proc_macro2::TokenStream,
    config: ParserConfig,
) -> Result<Vec<Node>> {
    let parser = move |input: ParseStream| Parser::new(config).parse(input);

    parser.parse2(tokens)
}
