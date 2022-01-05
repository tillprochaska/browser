use crate::cssom;
use crate::dom;
use std::collections::HashMap;
use std::vec::Vec;

const HIDDEN_ELEMENTS: [&str; 15] = [
    "area", "base", "basefont", "datalist", "head", "link", "meta", "noembed", "noframes", "param",
    "rp", "script", "style", "template", "title",
];

const BLOCK_ELEMENTS: [&str; 18] = [
    "address",
    "blockquote",
    "center",
    "dialog",
    "div",
    "figure",
    "figcaption",
    "footer",
    "form",
    "header",
    "hr",
    "legend",
    "listing",
    "main",
    "p",
    "plaintext",
    "pre",
    "xmp",
];

#[derive(PartialEq, Eq, Clone)]
pub struct RenderNode<'a> {
    pub node: &'a dom::Node,
    pub declarations: cssom::Declarations,
    pub children: RenderNodes<'a>,
}

impl<'a> RenderNode<'a> {
    pub fn from(node: &'a dom::Node, rulesets: &'a cssom::Rulesets) -> Self {
        if let None = node.element() {
            return RenderNode {
                node: node,
                declarations: HashMap::new(),
                children: Vec::new(),
            };
        }

        let element = node.element().unwrap();
        let children = element
            .children
            .iter()
            .map(|child| RenderNode::from(child, rulesets))
            .collect();

        let declarations = declarations_for_element(&element, &rulesets);

        return RenderNode {
            node: node,
            children: children,
            declarations: declarations,
        };
    }

    fn display_type(&self) -> DisplayType {
        let element = self.node.element();

        if let None = element {
            return DisplayType::Inline;
        }

        if let Some(value) = self.declarations.get("display") {
            match value {
                cssom::Value::String(value) if value == "none" => return DisplayType::None,
                cssom::Value::String(value) if value == "block" => return DisplayType::Block,
                cssom::Value::String(value) if value == "inline" => return DisplayType::Inline,
                _ => (),
            }
        }

        let tag = element.unwrap().tag.as_ref();

        if HIDDEN_ELEMENTS.contains(&tag) {
            return DisplayType::None;
        }

        if BLOCK_ELEMENTS.contains(&tag) {
            return DisplayType::Block;
        }

        return DisplayType::Inline;
    }
}

pub type RenderNodes<'a> = Vec<RenderNode<'a>>;

struct MatchedRuleset<'a> {
    selector: &'a cssom::Selector,
    declarations: &'a cssom::Declarations,
}

impl<'a> MatchedRuleset<'a> {
    pub fn new(selector: &'a cssom::Selector, declarations: &'a cssom::Declarations) -> Self {
        return Self {
            selector: selector,
            declarations: declarations,
        };
    }
}

#[derive(PartialEq, Eq)]
pub enum DisplayType {
    None,
    Block,
    Inline,
}

fn declarations_for_element(
    element: &dom::Element,
    rulesets: &cssom::Rulesets,
) -> cssom::Declarations {
    let mut matches = Vec::new();

    // Find matching rulesets and expand rulesets with multiple,
    // comma-separated selectors
    for ruleset in rulesets {
        for selector in &ruleset.selectors {
            if !element_matches_selector(element, selector) {
                continue;
            }

            let matched_ruleset = MatchedRuleset::new(&selector, &ruleset.declarations);
            matches.push(matched_ruleset);
        }
    }

    // Sort matching rulesets by selector specificity
    matches.sort_by(|a, b| {
        return a.selector.specificity().cmp(&b.selector.specificity());
    });

    // Merge declarations from all matching rulesets
    let mut declarations = cssom::Declarations::new();

    for ruleset in matches {
        for (property, value) in ruleset.declarations.iter() {
            declarations.insert(String::from(property), value.clone());
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
    use crate::cssom;
    use crate::dom::{Element, Node};
    use crate::html;

    #[test]
    fn test_render_node_from() {
        let rulesets = css::Parser::parse("h1, p { font-family: sans-serif; color: #333; } h1 { color: #000; } p { line-height: 20px; }");

        let nodes = html::Parser::parse("<h1>Hello World!</h1>");
        let h1 = RenderNode::from(&nodes[0], &rulesets);

        assert!(h1.node.element().unwrap().tag == "h1");
        assert!(h1.declarations.len() == 2);
        assert!(h1.declarations["font-family"] == cssom::Value::String("sans-serif".to_owned()));
        assert!(h1.declarations["color"] == cssom::Value::String("#000".to_owned()));

        let nodes = html::Parser::parse("<p>Hello World!</p>");
        let p = RenderNode::from(&nodes[0], &rulesets);

        assert!(p.node.element().unwrap().tag == "p");
        assert!(p.declarations.len() == 3);
        assert!(p.declarations["font-family"] == cssom::Value::String("sans-serif".to_owned()));
        assert!(p.declarations["color"] == cssom::Value::String("#333".to_owned()));
        assert!(
            p.declarations["line-height"] == cssom::Value::Numeric(cssom::NumericValue::Px(20))
        );
    }

    #[test]
    fn test_render_tree_node_recursive() {
        let nodes = html::Parser::parse("<div><p>Lorem ipsum</p></div>");
        let rulesets = css::Parser::parse("div { background: red; } p { color: yellow; }");

        let div = &RenderNode::from(&nodes[0], &rulesets);
        let p = &div.children[0];

        assert!(div.node.element().unwrap().tag == "div");
        assert!(div.declarations.len() == 1);

        assert!(p.node.element().unwrap().tag == "p");
        assert!(p.declarations.len() == 1);
    }

    #[test]
    fn test_declarations_for_element() {
        let element = &dom::Element::new("p");
        let rulesets = &css::Parser::parse("h1 { color: red; } p { color: #333; }");
        let declarations = declarations_for_element(element, rulesets);

        assert!(declarations.len() == 1);
        assert!(declarations["color"] == cssom::Value::String("#333".to_owned()));
    }

    #[test]
    fn test_declarations_for_element_specificity() {
        let element = &dom::Element::new("p").attr("id", "foo");
        let css = "p#foo { color: green; } #foo { color: red; } p { color: pink; }";
        let rulesets = &css::Parser::parse(css);
        let declarations = declarations_for_element(element, rulesets);

        assert!(declarations.len() == 1);
        assert!(declarations["color"] == cssom::Value::String("green".to_owned()));
    }

    #[test]
    fn test_declarations_for_element_multiple_selectors() {
        let element = &dom::Element::new("p").attr("class", "foo");
        let rulesets = &css::Parser::parse(".foo { color: green; } p, #bar { color: red; }");
        let declarations = declarations_for_element(element, rulesets);

        assert!(declarations.len() == 1);
        assert!(declarations["color"] == cssom::Value::String("red".to_owned()));
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

    #[test]
    fn test_render_node_display_type() {
        let rulesets = css::Parser::parse("");

        let meta_dom_node = Node::Element(Element::new("meta"));
        let div_dom_node = Node::Element(Element::new("div"));
        let span_dom_node = Node::Element(Element::new("span"));

        let mut meta_render_node = RenderNode::from(&meta_dom_node, &rulesets);
        let mut div_render_node = RenderNode::from(&div_dom_node, &rulesets);
        let mut span_render_node = RenderNode::from(&span_dom_node, &rulesets);

        assert!(meta_render_node.display_type() == DisplayType::None);
        assert!(div_render_node.display_type() == DisplayType::Block);
        assert!(span_render_node.display_type() == DisplayType::Inline);

        let overrides = css::Parser::parse("div { display: inline; } span { display: block; }");

        meta_render_node = RenderNode::from(&meta_dom_node, &overrides);
        div_render_node = RenderNode::from(&div_dom_node, &overrides);
        span_render_node = RenderNode::from(&span_dom_node, &overrides);

        assert!(meta_render_node.display_type() == DisplayType::None);
        assert!(div_render_node.display_type() == DisplayType::Inline);
        assert!(span_render_node.display_type() == DisplayType::Block);
    }
}
