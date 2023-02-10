use std::{
    f64::consts::PI,
    sync::{Arc, Mutex},
};

use crate::{
    curve::Curve,
    intersection::Intersection,
    lane::{Lane, LaneKind},
    property::{Property, PropertyKind}, TILE, road_profile::{RoadProfile},
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
        road_profile: Arc<Mutex<RoadProfile>>,
    ) -> Self {
        let width = road_profile.lock().unwrap().width();

        let mut i0_lock = i0.lock().unwrap();
        let mut i1_lock = i1.lock().unwrap();
        let mut i2_lock = i2.lock().unwrap();

        // Angles for connecting to Intersections
        let a0 = i0_lock.center.angle(&i2_lock.center);
        let a1 = a0 - PI / 2.0;
        let a2 = i2_lock.center.angle(&i0_lock.center);
        let a3 = a2 + PI / 2.0;
        
        // Define Road Central Curve
        let n0 = i0_lock.center.offset(a0, width);
        let n1 = i2_lock.center.offset(a2, width);
        let curve = Curve::new(n0, n1, a0, a2);
        
        // Get connections from Intersections
        let c0s = i0_lock.get_connections(a0, road_profile.clone());
        let c1s = i2_lock.get_connections(a2, road_profile.clone());

        // Add lanes to road
        let mut lanes = Vec::new();
        let mut offset = 0.0;
        let mut i = 0;
        for lane_kind in &road_profile.lock().unwrap().right_lane_kinds {
            let width = match lane_kind {
                LaneKind::Car => 4.0,
                LaneKind::Bike => 2.0,
                LaneKind::Pedestrian => 2.0,
            };
            offset += width / 2.0;
            let l0 = Arc::new(Mutex::new(Lane::new(
                c0s[i].clone(),
                c1s[c1s.len() - i - 1].clone(),
                curve.offset(offset),
                width,
                *lane_kind
            )));
            offset += width / 2.0;

            // Add lanes to lane list
            lanes.push(l0.clone());

            // Add lanes to connections
            c0s[i].lock().unwrap().out_lane.push(l0.clone());
            c1s[c1s.len() - i - 1].lock().unwrap().in_lane.push(l0.clone());
            i += 1;
        }
        
        let mut offset = 0.0;
        let mut i = 0;
        for lane_kind in &road_profile.lock().unwrap().left_lane_kinds {
            let width = match lane_kind {
                LaneKind::Car => 4.0,
                LaneKind::Bike => 2.0,
                LaneKind::Pedestrian => 2.0,
            };
            offset += width / 2.0;
            let l1 = Arc::new(Mutex::new(Lane::new(
                c1s[i].clone(),
                c0s[c0s.len() - i - 1].clone(),
                curve.reverse().offset(offset),
                width,
                *lane_kind
            )));
            offset += width / 2.0;

            // Add lanes to lane list
            lanes.push(l1.clone());
            
            // Add lanes to connections
            c1s[i].lock().unwrap().out_lane.push(l1.clone());
            c0s[c0s.len() - i - 1].lock().unwrap().in_lane.push(l1.clone());
            i += 1;
        }
        

        // Add Properties
        let mut properties = Vec::new();
        let length = curve.length();
        let plot_width = TILE * 4.0;
        let plot_depth = TILE * 4.0;
        let mut i = 0.0;

        while i <= length - plot_width {
            properties.push(Property::new(
                PropertyKind::Vacant,
                curve.n0.offset(a0, i).offset(a1, width / 2.0),
                curve.n0.offset(a0, i).offset(a1, width / 2.0 + plot_depth),
                curve
                    .n0
                    .offset(a0, i + plot_width)
                    .offset(a1, width / 2.0 + plot_depth),
                curve.n0.offset(a0, i + plot_width).offset(a1, width / 2.0),
            ));
            i += plot_width;
        }

        i = 0.0;
        while i <= length - plot_width {
            properties.push(Property::new(
                PropertyKind::Vacant,
                curve.n0.offset(a0, i).offset(a3 + PI, width / 2.0),
                curve.n0.offset(a0, i).offset(a3 + PI, width / 2.0 + plot_depth),
                curve.n0.offset(a0, i + plot_width).offset(a3 + PI, width / 2.0 + plot_depth),
                curve.n0.offset(a0, i + plot_width).offset(a3 + PI, width / 2.0),
            ));
            i += plot_width;
        }

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
