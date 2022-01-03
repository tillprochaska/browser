use crate::cssom;
use crate::dom;
use std::vec::Vec;

struct StyledTree {
    nodes: StyledNodes,
}

impl StyledTree {
    pub fn from(nodes: dom::Nodes, rulesets: cssom::Rulesets) -> Self {
        let nodes = nodes.iter().map(|node| {
            let declarations;

            if node.element().is_some() {
                declarations = declarations_for_element(node.element().unwrap(), &rulesets);
            } else {
                declarations = cssom::Declarations::new();
            }

            return StyledNode {
                node: node.clone(),
                declarations: declarations,
            };
        });

        return Self {
            nodes: nodes.collect(),
        };
    }
}

pub struct StyledNode {
    node: dom::Node,
    declarations: cssom::Declarations,
}

pub type StyledNodes = Vec<StyledNode>;

fn declarations_for_element(
    element: &dom::Element,
    rulesets: &cssom::Rulesets,
) -> cssom::Declarations {
    let matching_rulesets = rulesets.iter().filter(|ruleset| {
        return ruleset
            .selectors
            .iter()
            .any(&|selector: &cssom::Selector| element_matches_selector(element, selector));
    });

    let mut declarations = cssom::Declarations::new();

    for ruleset in matching_rulesets {
        for (property, value) in ruleset.declarations.iter() {
            declarations.insert(String::from(property), String::from(value));
        }
    }

    return declarations;
}

fn element_matches_selector(element: &dom::Element, selector: &cssom::Selector) -> bool {
    if let Some(tag) = &selector.tag {
        if *tag != element.tag {
            return false;
        }
    }

    if let (Some(selector_id), Some(attr_id)) = (&selector.id, element.attrs.get("id")) {
        if selector_id != attr_id {
            return false;
        }
    }

    for class in &selector.classes {
        if !element.class_list().contains(&class) {
            println!("{}", class);
            return false;
        }
    }

    for (name, value) in &selector.attrs {
        if let Some(element_attr) = element.attrs.get(name) {
            if element_attr == value {
                continue;
            }
        }

        return false;
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::css;
    use crate::html;

    #[test]
    fn test_declarations_for_node() {
        let nodes = html::Parser::parse("<h1>Hello World!</h1><p>Lorem ipsum</p>");
        let rulesets = css::Parser::parse("h1, p { font-family: sans-serif; color: #333; } h1 { color: #000; } p { line-height: 1.5; }");

        let styled_tree = StyledTree::from(nodes, rulesets);
        let h1 = &styled_tree.nodes[0];
        let p = &styled_tree.nodes[1];

        assert!(h1.node.element().unwrap().tag == "h1");
        assert!(h1.declarations.len() == 2);
        assert!(h1.declarations["font-family"] == "sans-serif");
        assert!(h1.declarations["color"] == "#000");

        assert!(p.node.element().unwrap().tag == "p");
        assert!(p.declarations.len() == 3);
        assert!(p.declarations["font-family"] == "sans-serif");
        assert!(p.declarations["color"] == "#333");
        assert!(p.declarations["line-height"] == "1.5");
    }

    #[test]
    fn test_element_matches_selector_tag() {
        let selector = &cssom::Selector::new().tag("p");
        let p = &dom::Element::new("p");
        let div = &dom::Element::new("div");

        assert!(element_matches_selector(p, selector) == true);
        assert!(element_matches_selector(div, selector) == false);
    }

    #[test]
    fn test_element_matches_selector_classes() {
        let selector = &cssom::Selector::new().class("foo").class("bar");

        let element = &dom::Element::new("div").attr("class", "foo bar baz");
        assert!(element_matches_selector(element, selector) == true);

        let element = &dom::Element::new("div").attr("class", "foo baz");
        assert!(element_matches_selector(element, selector) == false);
    }

    #[test]
    fn test_element_matches_selector_id() {
        let selector = &cssom::Selector::new().id("foo");

        let element = &dom::Element::new("div").attr("id", "foo");
        assert!(element_matches_selector(element, selector) == true);

        let element = &dom::Element::new("div").attr("id", "bar");
        assert!(element_matches_selector(element, selector) == false);
    }

    #[test]
    fn test_element_matches_selector_attrs() {
        let selector = &cssom::Selector::new().attr("foo", "bar").attr("bar", "baz");

        let element = &dom::Element::new("div")
            .attr("foo", "bar")
            .attr("bar", "baz")
            .attr("lorem", "ipsum");

        assert!(element_matches_selector(element, selector) == true);

        let element = &dom::Element::new("div")
            .attr("bar", "baz")
            .attr("lorem", "ipsum");

        assert!(element_matches_selector(element, selector) == false);
    }
}
