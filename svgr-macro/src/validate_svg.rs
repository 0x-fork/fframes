use crate::node::NodeName;
use syn::{parse::ParseBuffer, Result};

const UNSUPPORTED_NODES: [&'static str; 9] = [
    "altGlyph",
    "altGlyphDef",
    "animate",
    "animateColor",
    "animateMotion",
    "animateTransform",
    "hkern",
    "vkern",
    "foreignObject",
];

pub(crate) fn validate_node(input: &ParseBuffer, node: &NodeName) -> Result<()> {
    match node.to_string() {
        name if UNSUPPORTED_NODES.contains(&name.as_str()) => {
            Err(input.error(format!("Element <{} /> is not supported", name)))
        }
        _ => Ok(()),
    }
}

const UNSUPPORTED_ATTRS: [&'static str; 1] = ["dominant-baseline"];

pub(crate) fn validate_attribute(input: &ParseBuffer, node: &NodeName) -> Result<()> {
    match node.to_string() {
        name if name == "href" => {
            Err(input.error("href attribute is not supported, use xlink:href"))
        }
        name if UNSUPPORTED_ATTRS.contains(&name.as_str()) => {
            Err(input.error(format!("attribute {} is not supported", name)))
        }
        _ => Ok(()),
    }
}
