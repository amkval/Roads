use std::sync::{Arc, Mutex};

use cairo::Context;

use crate::{connection::Connection, curve::Curve, node::Node};

pub enum LaneKind {
    Car,
    Bike,
    Pedestrian,
}

pub struct Lane {
    pub c0: Arc<Mutex<Connection>>,
    pub c1: Arc<Mutex<Connection>>,
    pub curve: Curve,
    pub width: f64,
    pub kind: LaneKind,
}

impl Lane {
    pub fn new(
        c0: Arc<Mutex<Connection>>,
        c1: Arc<Mutex<Connection>>,
        curve: Curve,
        width: f64,
        kind: LaneKind,
    ) -> Self {
        Self {
            c0,
            c1,
            curve,
            width,
            kind,
        }
    }

    pub fn draw(&self, context: &Context) {
        let (width, (r, g, b)) = match &self.kind {
            LaneKind::Car => (4.0, (0.15, 0.13, 0.13)),
            LaneKind::Bike => (2.0, (0.53, 0.40, 0.38)),
            LaneKind::Pedestrian => (2.0, (0.33, 0.33, 0.36)),
        };


        // Offset Curves
        let curve1 = self.curve.offset(width / 2.0);
        let curve2 = self.curve.reverse().offset(width / 2.0);

        curve1.plot(context);
        context.line_to(curve2.n0.x, curve2.n0.y);

        curve2.plot(context);
        context.line_to(curve1.n0.x, curve1.n0.y);

        context.set_source_rgb(r, g, b);
        context.stroke_preserve().expect("omg!");
        context.fill().expect("omg!");

        // Draw Center Line
        //context.set_source_rgb(0.60, 0.20, 0.60);
        //self.curve.plot(context);
        //context.stroke().expect("Darn, you got me good!");
    }

    pub fn length(&self) -> f64 {
        self.curve.length()
    }

    pub fn position_at(&self, d: f64) -> Node {
        self.curve.position_at(d)
    }
}
