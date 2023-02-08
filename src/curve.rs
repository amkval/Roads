use std::f64::consts::PI;

use cairo::Context;

use crate::node::Node;

pub struct Curve {
    pub n0: Node,
    pub n1: Node,
    pub c: Node,
    pub is_curved: bool,
    pub is_reversed: bool,
}

impl Curve {
    pub fn new(n0: Node, n1: Node, a0: f64, a1: f64) -> Self {
        // Temporary Nodes for calculations
        let no0 = n0.offset(a0 + PI / 2.0, 10.0);
        let no1 = n1.offset(a1 + PI / 2.0, 10.0);

        // Find intersection of lines
        let a1 = no0.y - n0.y;
        let b1 = n0.x - no0.x;
        let c1 = a1 * n0.x + b1 * n0.y;

        let a2 = no1.y - n1.y;
        let b2 = n1.x - no1.x;
        let c2 = a2 * n1.x + b2 * n1.y;

        let mut determinant = a1 * b2 - a2 * b1;
        determinant = (determinant * 100.0).round() / 100.0;

        // Find the new center
        if determinant.abs() == 0.0 {
            let c = Node::new((n0.x + n1.x) / 2.0, (n0.y + n1.y) / 2.0);
            Self {
                n0,
                n1,
                c,
                is_curved: false,
                is_reversed: false,
            }
        } else {
            let x = (b2 * c1 - b1 * c2) / determinant;
            let y = (a1 * c2 - a2 * c1) / determinant;
            let c = Node::new(x, y);

            let d0 = ((n0.x - c.x).powf(2.0) + (n0.y - c.y).powf(2.0)).sqrt();
            let d1 = ((no0.x - c.x).powf(2.0) + (no0.y - c.y).powf(2.0)).sqrt();

            let is_reversed = d0 > d1;
            Self {
                n0,
                n1,
                c,
                is_curved: true,
                is_reversed,
            }
        }
    }

    pub fn new_1(n0: Node, n1: Node, c: Node, is_curved: bool, is_reversed: bool) -> Self {
        Self {
            n0,
            n1,
            c,
            is_curved,
            is_reversed,
        }
    }

    pub fn reverse(&self) -> Curve {
        Curve::new_1(self.n1, self.n0, self.c, self.is_curved, !self.is_reversed)
    }

    pub fn offset(&self, mut offset: f64) -> Curve {
        if self.is_curved {
            if self.is_reversed {
                offset = -offset;
            }
            let a0 = self.n0.get_angle(&self.c) + PI;
            let a1 = self.n1.get_angle(&self.c) + PI;
            let n0 = self.n0.offset(a0, offset);
            let n1 = self.n1.offset(a1, offset);
            let c = self.c.clone();
            Curve::new_1(n0, n1, c, self.is_curved, self.is_reversed)
        } else {
            let a = self.n0.get_angle(&self.c) - PI / 2.0;
            let n0 = self.n0.offset(a, offset);
            let n1 = self.n1.offset(a, offset);
            let c = self.c.offset(a, offset);
            Curve::new_1(n0, n1, c, self.is_curved, self.is_reversed)
        }
    }

    pub fn plot(&self, context: &Context) {
        if !self.is_curved {
            // The line is straight, draw a line
            context.move_to(self.n0.x, self.n0.y);
            context.line_to(self.n1.x, self.n1.y);
        } else {
            // Find radius o arc
            let radius =
                ((self.c.x - self.n0.x).powf(2.0) + (self.c.y - self.n0.y).powf(2.0)).sqrt();

            // Find start and stop angle of new arc
            let a0 = self.c.get_angle(&self.n0);
            let a1 = self.c.get_angle(&self.n1);

            if self.is_reversed {
                context.arc_negative(self.c.x, self.c.y, radius, a0, a1);
            } else {
                context.arc(self.c.x, self.c.y, radius, a0, a1);
            }
        }
    }

    pub fn length(&self) -> f64 {
        if self.is_curved {
            let a0 = self.c.get_angle(&self.n0);
            let a1 = self.c.get_angle(&self.n1);

            // TODO: Make this right!
            let da = if self.is_reversed {
                a0 - a1
            } else {
                a1 - a0
            };

            let radius = ((self.c.x - self.n0.x).powf(2.0) + (self.c.y - self.n0.y).powf(2.0)).sqrt();
            (da / 360.0) * 2.0 * PI * radius
        } else {
            let dx = (self.n0.x - self.n1.x).abs();
            let dy = (self.n0.y - self.n1.y).abs();   
            (dx.powi(2) + dy.powi(2)).sqrt()
        }
    }

    pub fn position_at(&self, d: f64) -> Node {
        if self.is_curved {
            let a0 = self.c.get_angle(&self.n0);
            let radius = ((self.c.x - self.n0.x).powf(2.0) + (self.c.y - self.n0.y).powf(2.0)).sqrt();
            let a = d/radius;
            self.c.offset(a0 + a, radius)
        } else {
            let d0 = ((self.n0.x - self.n1.x).powf(2.0) + (self.n0.y - self.n1.y).powf(2.0)).sqrt();
            let t = d / d0;
            Node::new((1.0 - t) * self.n0.x + t * self.n1.x, (1.0 - t) * self.n0.y + t * self.n1.y)
        }
    }
}
