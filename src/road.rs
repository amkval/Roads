use std::{f64::consts::PI, rc::Rc, cell::RefCell};

use crate::{
    curve::Curve,
    intersection::Intersection,
    lane::Lane,
    property::{Property, PropertyKind}, connection::{Connection, ConnectionKind},
};
use cairo::Context;

pub struct Road {
    pub curve: Curve,
    pub width: f64,
    pub properties: Vec<Property>,
}

impl Road {
    pub fn new(i0: Rc<RefCell<Intersection>>, i1: Rc<RefCell<Intersection>>, i2: Rc<RefCell<Intersection>>, width: f64) -> Self {
        
        // Angles for connecting to Intersections
        let a0 = i0.borrow().center.get_angle(&i2.borrow().center);
        let a1 = a0 - PI / 2.0;
        let a2 = i2.borrow().center.get_angle(&i0.borrow().center);
        let a3 = a2 + PI / 2.0;

        // Define Road Central Curve
        let n0 = i0.borrow().center.offset(a0, width*1.5);
        let n1 = i2.borrow().center.offset(a2, width*1.5);

        let curve = Curve::new(n0, n1, a1, a3);

        // Add connections to intersections
        let n0 = i0.borrow().center.offset(a0, width*1.5).offset(a1, width/2.0);
        let n2 = i2.borrow().center.offset(a2, width*1.5).offset(a3, width/2.0);
        i0.borrow_mut().add_connection(Connection::new(n0, ConnectionKind::Out, a0, width/2.));
        i2.borrow_mut().add_connection(Connection::new(n2, ConnectionKind::In, a2, width/2.));

        let nr0 = i0.borrow().center.offset(a0, width*1.5).offset(a1 + PI, width/2.0);
        let nr2 = i2.borrow().center.offset(a2, width*1.5).offset(a3 - PI, width/2.0);
        i0.borrow_mut().add_connection(Connection::new(nr0, ConnectionKind::In, a0, -width/2.));
        i2.borrow_mut().add_connection(Connection::new(nr2, ConnectionKind::Out, a2, -width/2.));

        // Add Properties
        let length = curve.length();
        let plot_width = 100.0;
        let plot_depth = 100.0;
        let mut properties = Vec::new();
        let mut i = 0.0;

        while i < length - width - plot_width {
            properties.push(Property::new(
                PropertyKind::Residential,
                curve.n0.offset(a0, i).offset(a1, width),
                curve
                    .n0
                    .offset(a0, i)
                    .offset(a1, width + plot_depth),
                curve
                    .n0
                    .offset(a0, i + plot_width)
                    .offset(a1, width + plot_depth),
                curve
                    .n0
                    .offset(a0, i + plot_width)
                    .offset(a1, width),
            ));
            i += plot_width;
        }

        Self {
            curve,
            width,
            properties,
        }
    }

    pub fn draw(&self, context: &Context) {
        let lane1 = Lane::new(self.curve.offset(10.0), self.width / 2.0);
        let lane2 = Lane::new(self.curve.reverse().offset(10.0), self.width / 2.0);
        lane1.draw(context);
        lane2.draw(context);

        /*
        for property in &self.properties {
            property.draw(context);
        }
        */
    }
}
