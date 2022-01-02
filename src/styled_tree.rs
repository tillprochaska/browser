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
        // For now, we implement selectors consisting of a single tag name only
        return ruleset.selectors.iter().any(&|selector: &cssom::Selector| {
            return selector.text == element.name;
        });
    });

    let mut declarations = cssom::Declarations::new();

    for ruleset in matching_rulesets {
        for (property, value) in ruleset.declarations.iter() {
            declarations.insert(String::from(property), String::from(value));
        }
    }

    return declarations;
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

        assert!(h1.node.element().unwrap().name == "h1");
        assert!(h1.declarations.len() == 2);
        assert!(h1.declarations["font-family"] == "sans-serif");
        assert!(h1.declarations["color"] == "#000");

        assert!(p.node.element().unwrap().name == "p");
        assert!(p.declarations.len() == 3);
        assert!(p.declarations["font-family"] == "sans-serif");
        assert!(p.declarations["color"] == "#333");
        assert!(p.declarations["line-height"] == "1.5");
    }
}
