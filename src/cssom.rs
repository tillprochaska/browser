use std::collections::HashMap;
use std::vec::Vec;

pub struct Ruleset {
    pub selectors: Selectors,
    pub declarations: Declarations,
}

impl Ruleset {
    pub fn new(selectors: Selectors, declarations: Declarations) -> Self {
        return Self {
            selectors: selectors,
            declarations: declarations,
        };
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct Selector {
    pub tag: Option<String>,
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub attrs: HashMap<String, String>,
}

impl Selector {
    pub fn new() -> Self {
        return Selector {
            tag: None,
            id: None,
            classes: Vec::new(),
            attrs: HashMap::new(),
        };
    }

    pub fn tag(mut self, tag: &str) -> Self {
        self.tag = Some(String::from(tag));

        return self;
    }

    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(String::from(id));

        return self;
    }

    pub fn class(mut self, class: &str) -> Self {
        self.classes.push(String::from(class));

        return self;
    }

    pub fn attr(mut self, name: &str, value: &str) -> Self {
        self.attrs.insert(String::from(name), String::from(value));

        return self;
    }
}

pub type Rulesets = Vec<Ruleset>;
pub type Selectors = Vec<Selector>;
pub type Declarations = HashMap<String, String>;
