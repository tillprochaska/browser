use core::cmp::Ordering;
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

    pub fn specificity(&self) -> Specificity {
        return Specificity {
            ids: if self.id.is_some() { 1 } else { 0 },
            classes: self.classes.len() + self.attrs.len(),
            elements: if self.tag.is_some() { 1 } else { 0 },
        };
    }
}

pub struct Specificity {
    ids: usize,
    classes: usize,
    elements: usize,
}

impl Specificity {
    pub fn new(ids: usize, classes: usize, elements: usize) -> Self {
        return Specificity {
            ids: ids,
            classes: classes,
            elements: elements,
        };
    }
}

impl PartialEq for Specificity {
    fn eq(&self, other: &Self) -> bool {
        return self.ids == other.ids
            && self.classes == other.classes
            && self.elements == other.elements;
    }
}

impl Eq for Specificity {}

impl PartialOrd for Specificity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for Specificity {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.ids != other.ids {
            return self.ids.cmp(&other.ids);
        }

        if self.classes != other.classes {
            return self.classes.cmp(&other.classes);
        }

        return self.elements.cmp(&other.elements);
    }
}

#[derive(PartialEq, Eq, Clone)]
pub enum Value {
    String(String),
    Numeric(NumericValue),
}

#[derive(PartialEq, Eq, Clone)]
pub enum NumericValue {
    Zero,
    Px(usize),
    Percentage(usize),
}

pub type Rulesets = Vec<Ruleset>;
pub type Selectors = Vec<Selector>;
pub type Declarations = HashMap<String, Value>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_specificity_eq() {
        let one = &Specificity::new(1, 1, 1);
        let two = &Specificity::new(1, 1, 1);
        let three = &Specificity::new(2, 1, 1);

        assert!(one.eq(two) == true);
        assert!(one.eq(three) == false);
        assert!(two.eq(three) == false);
    }

    #[test]
    fn test_specificity_cmp() {
        let greater = &Specificity::new(1, 0, 0);
        let less = &Specificity::new(0, 1000, 1000);

        assert!(greater.cmp(less) == Ordering::Greater);

        let greater = &Specificity::new(0, 1, 0);
        let less = &Specificity::new(0, 0, 1000);

        assert!(greater.cmp(less) == Ordering::Greater);

        let greater = &Specificity::new(0, 0, 1);
        let less = &Specificity::new(0, 0, 0);

        assert!(greater.cmp(less) == Ordering::Greater);
    }
}
