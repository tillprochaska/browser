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

        node.calculate_position(&anchor);

        // Calculate initial dimensions without child nodes
        node.calculate_dimensions(&viewport);

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
        node.calculate_dimensions(&viewport);

        return node;
    }

    fn calculate_dimensions(&mut self, containing_block_dimensions: &Dimensions) -> &Self {
        let computed_width;
        let computed_height;

        if let Some(width) = self.node.declarations.get("width") {
            // Explicit width given, supports px values only for now
            computed_width = width.strip_suffix("px").unwrap().parse().unwrap();
        } else {
            // Implicit width based on containing block
            computed_width = containing_block_dimensions.width;
        }

        if let Some(height) = self.node.declarations.get("height") {
            // Explicit height given, supports px values only for now
            computed_height = height.strip_suffix("px").unwrap().parse().unwrap();
        } else {
            // Implicit height based on children’s heights
            computed_height = self
                .children
                .iter()
                .fold(0u16, |acc, child| acc + child.dimensions.height);
        }

        self.dimensions = Dimensions::new(computed_width, computed_height);

        return self;
    }

    fn calculate_position(&mut self, anchor: &Point) -> &Self {
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
    fn test_layout_node_calculate_dimensions_width() {
        let viewport = Dimensions::new(640, 480);
        let anchor = Point::new(0, 0);

        let rulesets = css::Parser::parse("div { width: 320px; height: 240px; }");
        let dom = html::Parser::parse("<div><p></p></div>");
        let render_node = render_tree::RenderNode::from(&dom[0], &rulesets);

        let parent = &LayoutNode::from(&render_node, &viewport, &anchor);
        let child = &parent.children[0];

        assert!(parent.dimensions == Dimensions::new(320, 240));
        assert!(child.dimensions == Dimensions::new(320, 0));
    }

    #[test]
    fn test_layout_node_calculate_dimensions_height() {
        let viewport = Dimensions::new(640, 480);
        let anchor = Point::new(0, 0);

        let rulesets = css::Parser::parse("div { height: 100px; }");
        let dom = html::Parser::parse("<html><div></div><div></div></html>");
        let render_node = render_tree::RenderNode::from(&dom[0], &rulesets);
        let layout_node = &LayoutNode::from(&render_node, &viewport, &anchor);

        assert!(layout_node.dimensions == Dimensions::new(640, 200));
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
