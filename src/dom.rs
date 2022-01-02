use std::collections::HashMap;
use std::vec::Vec;

pub fn text(text: &str) -> Node {
    return Node::Text(String::from(text));
}

pub fn element(name: &str, children: Vec<Node>, attrs: AttrMap) -> Node {
    return Node::Element(Element {
        name: String::from(name),
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
    pub name: String,
    pub children: Vec<Node>,
    pub attrs: AttrMap,
}

pub type AttrMap = HashMap<String, String>;
pub type Nodes = Vec<Node>;
