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
        let mut curve = Curve::new(i0.borrow().center, i1.borrow().center, i2.borrow().center);

        let angle1 = curve.n0.get_angle(&curve.n2);
        let angle2 = angle1 + PI / 2.0;
        let angle3 = curve.n2.get_angle(&curve.n0);
        let angle4 = angle3 + PI / 2.0;

        let n0 = i0.borrow().center.offset(angle1, width).offset(angle4, width/2.0);
        let n2 = i2.borrow().center.offset(angle3, width).offset(angle4, width/2.0);
        i0.borrow_mut().add_connection(Connection::new(n0, ConnectionKind::In, angle1));
        i2.borrow_mut().add_connection(Connection::new(n2, ConnectionKind::In, angle3));

        curve.n0 = curve.n0.offset(angle1, width);
        curve.n2 = curve.n2.offset(angle3, width);

        let length = curve.length();
        let plot_width = 100.0;
        let plot_depth = 100.0;
        let mut properties = Vec::new();
        let mut i = 0.0;

        while i < length - width - plot_width {
            properties.push(Property::new(
                PropertyKind::Residential,
                curve.n0.offset(angle1, i).offset(angle2, width),
                curve
                    .n0
                    .offset(angle1, i)
                    .offset(angle2, width + plot_depth),
                curve
                    .n0
                    .offset(angle1, i + plot_width)
                    .offset(angle2, width + plot_depth),
                curve
                    .n0
                    .offset(angle1, i + plot_width)
                    .offset(angle2, width),
            ));
            properties.push(Property::new(
                PropertyKind::Commercial,
                curve.n2.offset(angle3, i).offset(angle4, width),
                curve
                    .n2
                    .offset(angle3, i)
                    .offset(angle4, width + plot_depth),
                curve
                    .n2
                    .offset(angle3, i + plot_width)
                    .offset(angle4, width + plot_depth),
                curve
                    .n2
                    .offset(angle3, i + plot_width)
                    .offset(angle4, width),
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
        //self.curve.n0.draw(context, self.width);
        //self.curve.n1.draw(context, self.width);
        //self.curve.n2.draw(context, self.width);

        let lane1 = Lane::new(self.curve.offset(11.0), self.width / 2.0);
        let lane2 = Lane::new(self.curve.reverse().offset(11.0), self.width / 2.0);
        lane1.draw(context);
        lane2.draw(context);

        for property in &self.properties {
            property.draw(context);
        }
    }
}
