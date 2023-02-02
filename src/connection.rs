use crate::node::Node;

pub enum ConnectionKind {
    In,
    Out,
}

pub struct Connection {
    pub center: Node,
    pub kind: ConnectionKind,
    pub angle: f64,
}

impl Connection {
    pub fn new(center: Node, kind: ConnectionKind, angle: f64) -> Self {
        Self {center, kind, angle}
    }
}
