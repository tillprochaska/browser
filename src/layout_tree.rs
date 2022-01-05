use crate::cssom;
use crate::render_tree;

#[derive(PartialEq, Eq, Clone)]
pub struct LayoutNode<'a> {
    node: &'a render_tree::RenderNode<'a>,
    children: Vec<Self>,
    dimensions: Dimensions,
    position: Point,
}

impl<'a> LayoutNode<'a> {
    fn from(
        render_node: &'a render_tree::RenderNode,
        viewport: &Dimensions,
        anchor: &Point,
    ) -> Self {
        let mut node = LayoutNode {
            node: render_node,
            children: Vec::new(),
            dimensions: *viewport,
            position: *anchor,
        };

        node.set_position(&anchor);

        // Calculate initial dimensions without child nodes
        node.set_dimensions(&viewport);

        let mut next_position = node.position;

        for render_child in &render_node.children {
            let layout_child = Self::from(&render_child, &node.dimensions, &next_position);

            next_position = Point::new(
                layout_child.position.x,
                layout_child.position.y + layout_child.dimensions.height,
            );

            node.children.push(layout_child);
        }

        // Calculate final dimensions including child nodes
        node.set_dimensions(&viewport);

        return node;
    }

    fn set_dimensions(&mut self, containing_block_dimensions: &Dimensions) -> &Self {
        self.dimensions.width = self.calculate_width(containing_block_dimensions);
        self.dimensions.height = self.calculate_height(containing_block_dimensions);

        return self;
    }

    fn calculate_width(&self, containing_block_dimensions: &Dimensions) -> u16 {
        let implicit_width = containing_block_dimensions.width;

        if self.node.declarations.get("width").is_none() {
            return implicit_width;
        }

        if let cssom::Value::Numeric(width) = &self.node.declarations["width"] {
            return match width {
                cssom::NumericValue::Zero => 0,
                cssom::NumericValue::Px(number) => *number,
                cssom::NumericValue::Percentage(number) => {
                    number * containing_block_dimensions.width / 100
                }
            };
        }

        return implicit_width;
    }

    fn calculate_height(&self, containing_block_dimensions: &Dimensions) -> u16 {
        let implicit_height = self
            .children
            .iter()
            .fold(0u16, |acc, child| acc + child.dimensions.height);

        if self.node.declarations.get("height").is_none() {
            return implicit_height;
        }

        if let cssom::Value::Numeric(height) = &self.node.declarations["height"] {
            return match height {
                cssom::NumericValue::Zero => 0,
                cssom::NumericValue::Px(number) => *number,
                cssom::NumericValue::Percentage(number) => {
                    number * containing_block_dimensions.height / 100
                }
            };
        }

        return implicit_height;
    }

    fn set_position(&mut self, anchor: &Point) -> &Self {
        // TODO: Support margin, position etc.
        self.position = Point::new(anchor.x, anchor.y);

        return self;
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Dimensions {
    width: u16,
    height: u16,
}

impl Dimensions {
    pub fn new(width: u16, height: u16) -> Self {
        return Self {
            width: width,
            height: height,
        };
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Point {
    x: u16,
    y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        return Self { x: x, y: y };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::css;
    use crate::html;
    use crate::render_tree;

    #[test]
    fn test_layout_node_from() {
        let viewport = Dimensions::new(640, 480);
        let anchor = Point::new(0, 0);

        let rulesets = css::Parser::parse("");
        let dom = html::Parser::parse("<main><h1>Hello World!</h1><p>Lorem ipsum</p></main>");

        let render_node = render_tree::RenderNode::from(&dom[0], &rulesets);
        let node = LayoutNode::from(&render_node, &viewport, &anchor);

        let h1 = &node.children[0];
        let p = &node.children[1];

        assert!(node.children.len() == 2);
        assert!(h1.children.len() == 1);
        assert!(p.children.len() == 1);
    }

    #[test]
    fn test_layout_node_set_dimensions_width() {
        let viewport = Dimensions::new(640, 480);
        let anchor = Point::new(0, 0);
        let dom = html::Parser::parse("<div></div>");

        // Implicit
        let implicit_rulesets = css::Parser::parse("");
        let implicit_render_node = render_tree::RenderNode::from(&dom[0], &implicit_rulesets);
        let implicit_layout_node = LayoutNode::from(&implicit_render_node, &viewport, &anchor);
        assert!(implicit_layout_node.dimensions.width == 640);

        // Zero
        let zero_rulesets = css::Parser::parse("div { width: 0; }");
        let zero_render_node = render_tree::RenderNode::from(&dom[0], &zero_rulesets);
        let zero_layout_node = LayoutNode::from(&zero_render_node, &viewport, &anchor);
        assert!(zero_layout_node.dimensions.width == 0);

        // Pixels
        let px_rulesets = css::Parser::parse("div { width: 50px; }");
        let px_render_node = render_tree::RenderNode::from(&dom[0], &px_rulesets);
        let px_layout_node = LayoutNode::from(&px_render_node, &viewport, &anchor);
        assert!(px_layout_node.dimensions.width == 50);

        // Percentage
        let percentage_rulesets = css::Parser::parse("div { width: 50%; }");
        let percentage_render_node = render_tree::RenderNode::from(&dom[0], &percentage_rulesets);
        let percentage_layout_node = LayoutNode::from(&percentage_render_node, &viewport, &anchor);
        println!("{}", percentage_layout_node.dimensions.width);
        assert!(percentage_layout_node.dimensions.width == 320);
    }

    #[test]
    fn test_layout_node_set_dimensions_height() {
        let viewport = Dimensions::new(640, 480);
        let anchor = Point::new(0, 0);
        let dom = html::Parser::parse("<div><p></p><p></p></div>");

        // Implicit
        let implicit_rulesets = css::Parser::parse("p { height: 100px; }");
        let implicit_render_node = render_tree::RenderNode::from(&dom[0], &implicit_rulesets);
        let implicit_layout_node = LayoutNode::from(&implicit_render_node, &viewport, &anchor);
        assert!(implicit_layout_node.dimensions.height == 200);

        // Zero
        let zero_rulesets = css::Parser::parse("div { height: 0; }");
        let zero_render_node = render_tree::RenderNode::from(&dom[0], &zero_rulesets);
        let zero_layout_node = LayoutNode::from(&zero_render_node, &viewport, &anchor);
        assert!(zero_layout_node.dimensions.height == 0);

        // Pixels
        let px_rulesets = css::Parser::parse("div { height: 50px; }");
        let px_render_node = render_tree::RenderNode::from(&dom[0], &px_rulesets);
        let px_layout_node = LayoutNode::from(&px_render_node, &viewport, &anchor);
        assert!(px_layout_node.dimensions.height == 50);

        // Percentage
        let percentage_rulesets = css::Parser::parse("div { height: 50%; }");
        let percentage_render_node = render_tree::RenderNode::from(&dom[0], &percentage_rulesets);
        let percentage_layout_node = LayoutNode::from(&percentage_render_node, &viewport, &anchor);
        assert!(percentage_layout_node.dimensions.height == 240);
    }

    #[test]
    fn test_layout_node_calculate_position() {
        let viewport = Dimensions::new(640, 480);
        let anchor = Point::new(0, 0);

        let rulesets = css::Parser::parse("div { height: 100px; }");
        let dom = html::Parser::parse("<html><div></div><div></div></html>");
        let render_node = render_tree::RenderNode::from(&dom[0], &rulesets);
        let layout_node = &LayoutNode::from(&render_node, &viewport, &anchor);

        assert!(layout_node.children[0].position == Point::new(0, 0));
        assert!(layout_node.children[1].position == Point::new(0, 100));
    }
}
