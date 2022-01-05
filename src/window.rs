extern crate minifb;

use crate::layout;
use crate::painting;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

pub struct Window {
    buffer: painting::WindowBuffer,
    inner: minifb::Window,
}

impl Window {
    pub fn new() -> Self {
        let mut inner =
            minifb::Window::new("Title", WIDTH, HEIGHT, minifb::WindowOptions::default()).unwrap();

        // Limit updates to 60fps
        inner.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        return Self {
            buffer: painting::WindowBuffer::new(WIDTH, HEIGHT),
            inner: inner,
        };
    }

    pub fn is_open(&self) -> bool {
        return self.inner.is_open();
    }

    pub fn paint_node(&mut self, node: &layout::LayoutNode) -> &Self {
        self.buffer.paint_node(node);

        return self;
    }

    pub fn update(&mut self) -> &Self {
        self.inner
            .update_with_buffer(&self.buffer.as_vec(), WIDTH, HEIGHT)
            .unwrap();

        return self;
    }
}
