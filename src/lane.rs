use std::sync::{Arc, Mutex};

use cairo::Context;

use crate::{connection::Connection, curve::Curve, node::Node};

pub struct Lane {
    pub c0: Arc<Mutex<Connection>>,
    pub c1: Arc<Mutex<Connection>>,
    pub curve: Curve,
    pub width: f64,
}

impl Lane {
    pub fn new(
        c0: Arc<Mutex<Connection>>,
        c1: Arc<Mutex<Connection>>,
        curve: Curve,
        width: f64,
    ) -> Self {
        Self {
            c0,
            c1,
            curve,
            width,
        }
    }

    pub fn draw(&self, context: &Context) {
        // Offset Curves
        context.set_source_rgb(0.20, 0.20, 0.20);
        let curve1 = self.curve.offset(self.width);
        let curve2 = self.curve.reverse().offset(self.width);

        curve1.plot(context);
        context.line_to(curve2.n0.x, curve2.n0.y);

        curve2.plot(context);
        context.line_to(curve1.n0.x, curve1.n0.y);

        context.set_source_rgb(0.50, 0.50, 0.50);
        context.stroke_preserve().expect("OMG!");
        context.fill().expect("Woops! Draw failed!");

        // Draw Center Line
        context.set_source_rgb(0.60, 0.20, 0.60);
        self.curve.plot(context);
        context.stroke().expect("Darn, you got me good!");
    }

    pub fn length(&self) -> f64 {
        self.curve.length()
    }

    pub fn position_at(&self, d: f64) -> Node {
        self.curve.position_at(d)
    }
}
