use std::f64::consts::PI;

use cairo::Context;

use crate::node::Node;

pub struct Curve {
    pub n0: Node,
    pub n1: Node,
    pub n2: Node,
}

impl Curve {
    pub fn new(n0: Node, n1: Node, n2: Node) -> Self {
        Self { n0, n1, n2 }
    }

    pub fn reverse(&self) -> Curve {
        Curve::new( self.n2, self.n1, self.n0 )
    }

    pub fn offset(&self, offset: f64) -> Curve {
        let a0 = self.n0.get_angle(&self.n1) + PI / 2.;
        let a1 = self.n0.get_angle(&self.n2) + PI / 2.;
        let a2 = self.n1.get_angle(&self.n2) + PI / 2.;
        let n0 = self.n0.offset(a0, offset);
        let n1 = self.n1.offset(a1, offset);
        let n2 = self.n2.offset(a2, offset);
        Curve::new(n0, n1, n2)
    }

    pub fn plot(&self, context: &Context) {
        quadratic_to(context, &self.n0, &self.n1, &self.n2);
    }
}

pub fn quadratic_to(context: &Context, n0: &Node, n1: &Node, n2: &Node) {
    context.curve_to(
        (n0.x + 2.0 * n1.x) / 3.0,
        (n0.y + 2.0 * n1.y) / 3.0,
        (n2.x + 2.0 * n1.x) / 3.0,
        (n2.y + 2.0 * n1.y) / 3.0,
        n2.x,
        n2.y,
    );
}
