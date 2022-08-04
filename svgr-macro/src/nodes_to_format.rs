use crate::node::{Node, NodeType};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{
    punctuated::Punctuated, Expr, ExprPath, ExprReference, Path, PathArguments, PathSegment,
};

pub(crate) fn prepare_svg_nodes_for_format_statement(
    nodes: Vec<Node>,
) -> (String, Vec<Expr>, Vec<TokenStream>) {
    
    let mut out = String::new();
    let mut values = vec![];
    let mut animations = vec![];

    for node in nodes {
        match node.node_type {
            NodeType::Element => {
                let name = node.name_as_string().unwrap();
                out.push_str(&format!("<{}", name));

                // attributes
                let (svg_string, attribute_values, attribute_animations) =
                    prepare_svg_nodes_for_format_statement(node.attributes);
                out.push_str(&svg_string);
                values.extend(attribute_values);
                animations.extend(attribute_animations);
                out.push('>');

                // children
                let (svg_string, children_values, child_animations) =
                    prepare_svg_nodes_for_format_statement(node.children);

                out.push_str(&svg_string);
                values.extend(children_values);
                animations.extend(child_animations);

                out.push_str(&format!("</{}>", name));
            }
            NodeType::Attribute => {
                out.push_str(&format!(" {}", node.name_as_string().unwrap()));
                if node.value.is_some() {
                    out.push_str(r#"="{}""#);
                    values.push(node.value.unwrap());
                }
            }
            NodeType::Text | NodeType::Block => {
                out.push_str("{}");
                values.push(node.value.unwrap());
            }
            NodeType::LazyTimelineBlock => {
                out.push_str(&format!(" {}", node.name_as_string().unwrap()));
                out.push_str(r#"="{}""#);

                let mut punctuated = Punctuated::new();
                let block = node.value.unwrap();

                punctuated.push(PathSegment {
                    ident: Ident::new(
                        &format!("ANIMATION_{}", uuid::Uuid::new_v4().to_simple()),
                        Span::call_site(),
                    ),
                    arguments: PathArguments::None,
                });

                let identifier = syn::Expr::Path(ExprPath {
                    path: Path {
                        leading_colon: None,
                        segments: punctuated,
                    },
                    attrs: vec![],
                    qself: None,
                });

                let mut macro_call = None;
                let block = match block {
                    Expr::Block(val) => match &val.block.stmts[0] {
                        syn::Stmt::Expr(expr) => match expr {
                            Expr::MethodCall(method) if method.method.to_string() == "animate" => {
                                macro_call = Some(method.args[0].clone());

                                let mut method_call_with_replaced_animation_ref = method.clone();
                                method_call_with_replaced_animation_ref.args = method
                                    .args
                                    .iter()
                                    .map(|_| {
                                        syn::Expr::Reference(ExprReference {
                                            and_token: Default::default(),
                                            raw: Default::default(),
                                            attrs: Default::default(),
                                            mutability: None,
                                            expr: Box::new(identifier.clone()),
                                        })
                                    })
                                    .collect();

                                Expr::MethodCall(method_call_with_replaced_animation_ref)
                            }
                            expr => expr.to_owned(),
                        },
                        _ => Expr::Block(val),
                    },
                    expr => expr,
                };

                if let Some(macro_call) = macro_call {
                    animations.push(quote! {
                        static ref #identifier: fframes::Animation::SteppedAnimation = #macro_call;
                    });
                }

                values.push(block);
            }
        }
    }

    (out, values, animations)
}
