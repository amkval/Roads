use std::f64::consts::PI;

use cairo::Context;

#[derive(Copy, Clone)]
pub struct Node {
    pub x: f64,
    pub y: f64,
}

impl Node {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn get_angle(&self, node: &Node) -> f64 {
        let dx = node.x - self.x;
        let dy = node.y - self.y;
        let mut a = dy.atan2(dx);
        if a < 0.0 {
            a += PI * 2.0;
        }
        a
    }

    pub fn offset(&self, angle: f64, offset: f64) -> Node {
        let x = self.x + angle.cos() * offset;
        let y = self.y + angle.sin() * offset;
        Node::new(x, y)
    }

    pub fn draw(&self, context: &Context, width: f64) {
        context.set_source_rgb(0.0, 0.0, 0.0);
        context.arc(self.x, self.y, width, 0.0, PI * 2.0);
        context.stroke().expect("Woops! Draw failed!");
    }
}