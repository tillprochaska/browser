use std::collections::HashMap;
use std::vec::Vec;

pub fn text(text: &str) -> Node {
    return Node::Text(String::from(text));
}

pub fn element(tag: &str, children: Vec<Node>, attrs: AttrMap) -> Node {
    return Node::Element(Element {
        tag: String::from(tag),
        children: children,
        attrs: attrs,
    });
}

#[derive(PartialEq, Eq, Clone)]
pub enum Node {
    Text(String),
    Element(Element),
}

impl Node {
    pub fn text(&self) -> Option<String> {
        return match self {
            Node::Text(text) => Some(String::from(text)),
            _ => None,
        };
    }

    pub fn element(&self) -> Option<&Element> {
        return match self {
            Node::Element(element) => Some(element),
            _ => None,
        };
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct Element {
    pub tag: String,
    pub children: Nodes,
    pub attrs: AttrMap,
}

impl Element {
    pub fn tag(tag: &str) -> Self {
        return Element {
            tag: String::from(tag),
            children: Nodes::new(),
            attrs: AttrMap::new(),
        };
    }
}

pub type AttrMap = HashMap<String, String>;
pub type Nodes = Vec<Node>;
