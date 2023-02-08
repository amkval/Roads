use std::{
    f64::consts::PI,
    sync::{Arc, Mutex},
};

use crate::{
    curve::Curve,
    intersection::Intersection,
    lane::Lane,
    property::{Property, PropertyKind},
};

use cairo::Context;

pub enum RoadKind {
    Normal,
}

pub struct Road {
    pub i0: Arc<Mutex<Intersection>>,
    pub i1: Arc<Mutex<Intersection>>,
    pub i2: Arc<Mutex<Intersection>>,
    pub curve: Curve,
    pub width: f64,
    pub properties: Vec<Property>,
    pub lanes: Vec<Arc<Mutex<Lane>>>,
}

impl Road {
    pub fn new(
        i0: Arc<Mutex<Intersection>>,
        i1: Arc<Mutex<Intersection>>,
        i2: Arc<Mutex<Intersection>>,
        width: f64,
    ) -> Self {
        let mut i0_lock = i0.lock().unwrap();
        let mut i1_lock = i1.lock().unwrap();
        let mut i2_lock = i2.lock().unwrap();

        // Angles for connecting to Intersections
        let a0 = i0_lock.center.get_angle(&i2_lock.center);
        let a1 = a0 - PI / 2.0;
        let a2 = i2_lock.center.get_angle(&i0_lock.center);
        let a3 = a2 + PI / 2.0;
        
        // Define Road Central Curve
        let n0 = i0_lock.center.offset(a0, width * 1.5);
        let n1 = i2_lock.center.offset(a2, width * 1.5);
        let curve = Curve::new(n0, n1, a1, a3);
        
        // Get connections from Intersections
        println!("Wololo!");
        let c0s = i0_lock.get_connections(a0, width);
        let c1s = i2_lock.get_connections(a2, width);
        println!("We got here, not!");

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
                curve.n0.offset(a0, i).offset(a1, width + plot_depth),
                curve
                    .n0
                    .offset(a0, i + plot_width)
                    .offset(a1, width + plot_depth),
                curve.n0.offset(a0, i + plot_width).offset(a1, width),
            ));
            i += plot_width;
        }

        // Add lanes to road
        let mut lanes = Vec::new();
        let l0 = Arc::new(Mutex::new(Lane::new(
            c0s.first().unwrap().clone(),
            c1s.last().unwrap().clone(),
            curve.offset(10.0),
            width / 2.0,
        )));
        lanes.push(l0.clone());
        let l1 = Arc::new(Mutex::new(Lane::new(
            c1s.first().unwrap().clone(),
            c0s.last().unwrap().clone(),
            curve.reverse().offset(10.0),
            width / 2.0,
        )));
        lanes.push(l1.clone());

        // Add lanes to connections...
        c0s.first().unwrap().lock().unwrap().out_lane.push(l0.clone());
        c0s.last().unwrap().lock().unwrap().in_lane.push(l1.clone());

        c1s.first().unwrap().lock().unwrap().out_lane.push(l1.clone());
        c1s.last().unwrap().lock().unwrap().in_lane.push(l0.clone());

        drop(i0_lock);
        drop(i1_lock);
        drop(i2_lock);

        // Create Road
        Self {
            i0,
            i1,
            i2,
            curve,
            width,
            properties,
            lanes,
        }
    }

    pub fn draw(&self, context: &Context) {
        for lane in &self.lanes {
            lane.lock().unwrap().draw(context);
        }

        for property in &self.properties {
            property.draw(context);
        }
    }
}
