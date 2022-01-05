use crate::cssom;
use crate::layout;

pub struct WindowBuffer {
    width: usize,
    height: usize,
    inner: Vec<u32>,
}

impl WindowBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        return Self {
            width: width,
            height: height,
            inner: vec![0xffffff; width * height],
        };
    }

    pub fn as_vec(&self) -> &Vec<u32> {
        return &self.inner;
    }

    pub fn paint_node(&mut self, node: &layout::LayoutNode) -> () {
        let background_color = node.node.declarations.get("background-color");

        if background_color.is_some() {
            if let cssom::Value::Color(background_color) = background_color.unwrap() {
                self.paint_rect(&node.position, &node.dimensions, background_color.as_u32());
            }
        }

        for child in &node.children {
            self.paint_node(child);
        }
    }

    fn paint_rect(
        &mut self,
        position: &layout::Point,
        dimensions: &layout::Dimensions,
        color: u32,
    ) -> () {
        let x1 = position.x;
        let x2 = position.x + dimensions.width;
        let y1 = position.y;
        let y2 = position.y + dimensions.height;

        for x in x1..x2 {
            for y in y1..y2 {
                self.paint_pixel(x, y, color);
            }
        }
    }

    fn paint_pixel(&mut self, x: usize, y: usize, color: u32) -> () {
        self.inner[y * self.width + x] = color;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::css;
    use crate::html;
    use crate::render;

    #[test]
    fn test_paint_node() {
        let dom = html::Parser::parse("<div><p></p></div>");
        let rulesets = css::Parser::parse(
            "
            div { width: 100%; height: 100%; background-color: #000; }
            p { width: 2px; height: 2px; background-color: #fff; }
        ",
        );

        let viewport = layout::Dimensions::new(5, 5);
        let anchor = layout::Point::new(0, 0);

        let render_node = render::RenderNode::from(&dom[0], &rulesets);
        let layout_node = layout::LayoutNode::from(&render_node, &viewport, &anchor);

        println!(
            "{}x{}",
            layout_node.dimensions.width, layout_node.dimensions.height
        );

        let mut buffer = WindowBuffer::new(5, 5);
        buffer.paint_node(&layout_node);

        #[rustfmt::skip]
        let expected = vec![
            0xffffff, 0xffffff, 0x0     , 0x0     , 0x0     ,
            0xffffff, 0xffffff, 0x0     , 0x0     , 0x0     ,
            0x0,      0x0     , 0x0     , 0x0     , 0x0     ,
            0x0,      0x0     , 0x0     , 0x0     , 0x0     ,
            0x0,      0x0     , 0x0     , 0x0     , 0x0     ,
        ];

        assert!(*buffer.as_vec() == expected);
    }

    #[test]
    fn test_paint_rect() {
        let mut buffer = WindowBuffer::new(5, 5);
        buffer.paint_rect(&layout::Point::new(2, 2), &layout::Dimensions::new(2, 2), 0);

        #[rustfmt::skip]
        let expected = vec![
            0xffffff, 0xffffff, 0xffffff, 0xffffff, 0xffffff,
            0xffffff, 0xffffff, 0xffffff, 0xffffff, 0xffffff,
            0xffffff, 0xffffff, 0x0     , 0x0     , 0xffffff,
            0xffffff, 0xffffff, 0x0     , 0x0     , 0xffffff,
            0xffffff, 0xffffff, 0xffffff, 0xffffff, 0xffffff,
        ];

        assert!(*buffer.as_vec() == expected);
    }

    #[test]
    fn test_paint_pixel() {
        let mut buffer = WindowBuffer::new(5, 5);
        buffer.paint_pixel(2, 2, 0);

        #[rustfmt::skip]
        let expected = vec![
            0xffffff, 0xffffff, 0xffffff, 0xffffff, 0xffffff,
            0xffffff, 0xffffff, 0xffffff, 0xffffff, 0xffffff,
            0xffffff, 0xffffff, 0x0,      0xffffff, 0xffffff,
            0xffffff, 0xffffff, 0xffffff, 0xffffff, 0xffffff,
            0xffffff, 0xffffff, 0xffffff, 0xffffff, 0xffffff,
        ];

        assert!(*buffer.as_vec() == expected);
    }
}
