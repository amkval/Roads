use std::sync::{Arc, Mutex};

use crate::{lane::{Lane, LaneKind}, node::Node};

#[derive(PartialEq, Eq)]
pub enum ConnectionKind {
    In,
    Out,
}

pub struct Connection {
    pub center: Node,
    pub kind: ConnectionKind,
    pub lane_kind: LaneKind,
    pub angle: f64,
    pub offset: f64,
    pub in_lane: Vec<Arc<Mutex<Lane>>>,
    pub out_lane: Vec<Arc<Mutex<Lane>>>,
}

impl Connection {
    pub fn new(center: Node, kind: ConnectionKind, lane_kind: LaneKind, angle: f64, offset: f64) -> Self {
        Self {
            center,
            kind,
            lane_kind,
            angle,
            offset,
            in_lane: Vec::new(),
            out_lane: Vec::new(),
        }
    }
}
