use std::{
    f64::consts::PI,
    sync::{Arc, Mutex},
};

use cairo::Context;

use crate::{
    connection::{Connection, ConnectionKind},
    curve::Curve,
    lane::{Lane, LaneKind, self},
    node::Node,
    road::Road, road_profile::{self, RoadProfile},
};

pub struct Intersection {
    pub center: Node,
    pub roads: Vec<Arc<Mutex<Road>>>,
    pub connections: Vec<Arc<Mutex<Connection>>>,
    pub lanes: Vec<Arc<Mutex<Lane>>>,
}

impl Intersection {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            center: Node::new(x, y),
            roads: Vec::new(),
            connections: Vec::new(),
            lanes: Vec::new(),
        }
    }

    pub fn draw(&self, context: &Context) {
        self.center.draw(context, 2.5);
        for lane in &self.lanes {
            lane.lock().unwrap().draw(context);
        }
    }

    pub fn get_connections(&mut self, a: f64, road_profile: Arc<Mutex<RoadProfile>>) -> Vec<Arc<Mutex<Connection>>> {
        let width = road_profile.lock().unwrap().width() / 2.0;
        let mut cs: Vec<Arc<Mutex<Connection>>> = Vec::new();
        
        let a0 = a + PI / 2.0;
        let a1 = a - PI / 2.0;
        
        let mut offset = 0.0;
        for lane_kind in &road_profile.lock().unwrap().right_lane_kinds {
            let lane_width = match lane_kind {
                LaneKind::Car => 4.0,
                LaneKind::Bike => 2.0,
                LaneKind::Pedestrian => 2.0,
            };
            offset += lane_width / 2.0;
            let n0 = self.center.offset(a, width * 2.0).offset(a0, offset);
            let c0 = Arc::new(Mutex::new(Connection::new(
                n0,
                ConnectionKind::Out,
                *lane_kind,
                a,
                width / 2.0,
            )));
            offset += lane_width / 2.0;
            cs.push(c0.clone());
            self.connections.push(c0.clone());
        }
        let mut offset = 0.0;
        for lane_kind in &road_profile.lock().unwrap().left_lane_kinds {
            let lane_width = match lane_kind {
                LaneKind::Car => 4.0,
                LaneKind::Bike => 2.0,
                LaneKind::Pedestrian => 2.0,               
            };
            offset += lane_width / 2.0;
            let n1 = self.center.offset(a, width * 2.0).offset(a1, offset);
            let c1 = Arc::new(Mutex::new(Connection::new(
                n1,
                ConnectionKind::In,
                *lane_kind,
                a,
                width / 2.0,
            )));
            offset += lane_width / 2.0;
            cs.push(c1.clone());
            self.connections.push(c1.clone());
        }

        self.add_lanes();
        cs
    }

    pub fn add_lanes(&mut self) {
        self.lanes.clear();
        // Setup lanes for new connection:
        for c0 in &self.connections {
            for c1 in &self.connections {
                if !Arc::ptr_eq(c0, c1) {
                    let mut c0_lock = c0.lock().unwrap();
                    let mut c1_lock = c1.lock().unwrap();
                    if c0_lock.kind == ConnectionKind::In && c1_lock.kind == ConnectionKind::Out {
                        if c0_lock.lane_kind == c1_lock.lane_kind {
                            let curve = Curve::new(
                                c0_lock.center,
                                c1_lock.center,
                                c0_lock.angle,
                                c1_lock.angle,
                            );
                            let l = Arc::new(Mutex::new(Lane::new(
                                c0.clone(),
                                c1.clone(),
                                curve,
                                5.0,
                                c0_lock.lane_kind,
                            )));
                            c0_lock.out_lane.push(l.clone());
                            c1_lock.in_lane.push(l.clone());
                            self.lanes.push(l);
                        }
                    }
                }
            }
        }
    }
}
