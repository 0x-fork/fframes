use crate::node::{Node, NodeType};
use syn::Expr;

pub(crate) fn nodes_to_format(nodes: Vec<Node>) -> (String, Vec<Expr>) {
    let mut out = String::new();
    let mut values = vec![];

    for node in nodes {
        match node.node_type {
            NodeType::Element => {
                let name = node.name_as_string().unwrap();
                out.push_str(&format!("<{}", name));

                // attributes
                let (svg_string, attribute_values) = nodes_to_format(node.attributes);
                out.push_str(&svg_string);
                values.extend(attribute_values);
                out.push('>');

                // children
                let (svg_string, children_values) = nodes_to_format(node.children);
                out.push_str(&svg_string);
                values.extend(children_values);

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
        }
    }

    (out, values)
}
