use cairo::Context;

use crate::node::Node;

pub enum PropertyKind {
    Vacant,
    Residential,
    Commercial,
    Industrial,
}

pub struct Property {
    kind: PropertyKind,
    n0: Node,
    n1: Node,
    n2: Node,
    n3: Node,
}

impl Property {
    pub fn new(kind: PropertyKind, n0: Node, n1: Node, n2: Node, n3: Node) -> Self {
        Self {
            kind,
            n0,
            n1,
            n2,
            n3,
        }
    }

    pub fn draw(&self, context: &Context) {
        match self.kind {
            PropertyKind::Residential => {
                context.set_source_rgba(0.0, 0.90, 0.0, 0.25);
            }
            PropertyKind::Commercial => {
                context.set_source_rgba(0.0, 0.0, 0.90, 0.25);
            }
            PropertyKind::Industrial => {
                context.set_source_rgba(0.0, 0.90, 0.90, 0.25);
            }
            PropertyKind::Vacant => {
                context.set_source_rgba(0.90, 0.90, 0.90, 0.25)
            },
        }

        context.move_to(self.n0.x, self.n0.y);
        context.line_to(self.n1.x, self.n1.y);
        context.line_to(self.n2.x, self.n2.y);
        context.line_to(self.n3.x, self.n3.y);
        context.close_path();
        context.stroke_preserve().expect("OMG!");
        context.fill().expect("OMG!");
    }
}
