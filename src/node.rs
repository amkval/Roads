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

    pub fn angle(&self, node: &Node) -> f64 {
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

    pub fn distance(&self, node: &Node) -> f64 {
        ((self.x - node.x).powf(2.0) + (self.y - node.y).powf(2.0)).sqrt().abs()
    }

    pub fn draw(&self, context: &Context, width: f64) {
        context.set_source_rgb(0.0, 0.0, 0.0);
        context.arc(self.x, self.y, width, 0.0, PI * 2.0);
        context.stroke().expect("Failed to draw Node!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angle() {
        let n0 = Node::new(10.0, 10.0);
        let n1 = Node::new(20.0, 10.0);
        assert!(n0.angle(&n1) == 0.0);
    }

    #[test]
    fn test_angle_2() {
        let n0 = Node::new(10.0, 10.0);
        let n1 = Node::new(0.0, 10.0);
        assert!(n0.angle(&n1) == PI);
    }

    #[test]
    fn test_offset_x() {
        let n0 = Node::new(10.0, 10.0);
        let n1 = n0.offset(0.0, 10.0);
        assert!(n1.x == 20.0);
    }

    #[test]
    fn test_offset_y() {
        let n0 = Node::new(10.0, 10.0);
        let n1 = n0.offset(PI / 2.0, 10.0);
        assert!(n1.y == 20.0);
    }
}