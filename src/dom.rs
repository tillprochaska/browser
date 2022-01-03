use std::collections::HashMap;
use std::vec::Vec;

pub fn text(text: &str) -> Node {
    return Node::Text(String::from(text));
}

pub fn element(tag: &str, children: Nodes, attrs: AttrMap) -> Node {
    let element = Element::new(tag).children(children).attrs(attrs);

    return Node::Element(element);
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
    pub fn new(tag: &str) -> Self {
        return Element {
            tag: String::from(tag),
            children: Nodes::new(),
            attrs: AttrMap::new(),
        };
    }

    pub fn children(mut self, children: Nodes) -> Self {
        self.children = children;

        return self;
    }

    pub fn attrs(mut self, attrs: AttrMap) -> Self {
        self.attrs = attrs;

        return self;
    }

    pub fn attr(mut self, name: &str, value: &str) -> Self {
        self.attrs.insert(String::from(name), String::from(value));

        return self;
    }

    pub fn class_list(&self) -> Vec<String> {
        if !self.attrs.contains_key("class") {
            return Vec::new();
        }

        return self
            .attrs
            .get("class")
            .unwrap()
            .split(&|c: char| c.is_whitespace())
            .map(&|class| String::from(class))
            .collect();
    }
}

pub type AttrMap = HashMap<String, String>;
pub type Nodes = Vec<Node>;
