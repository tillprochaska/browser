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

#[derive(PartialEq, Eq)]
pub struct Selector {
    pub text: String,
}

impl Selector {
    pub fn new(text: &str) -> Self {
        return Selector {
            text: String::from(text),
        };
    }
}

pub type Rulesets = Vec<Ruleset>;
pub type Selectors = Vec<Selector>;
pub type Declarations = HashMap<String, String>;
