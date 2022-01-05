use crate::cssom;
use crate::layout;

pub struct WindowBuffer {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
}

impl WindowBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        return Self {
            width: width,
            height: height,
            buffer: vec![0xffffff; width * height],
        };
    }

    pub fn as_vec(&self) -> &Vec<u32> {
        return &self.buffer;
    }

    pub fn paint_node(&mut self, node: &layout::LayoutNode) -> () {
        let background_color = node.node.declarations.get("background-color");

        if background_color.is_none() {
            return;
        }

        if let cssom::Value::Color(background_color) = background_color.unwrap() {
            self.paint_rect(&node.position, &node.dimensions, background_color.as_u32());
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
        self.buffer[y * self.width + x] = color;
    }
}
