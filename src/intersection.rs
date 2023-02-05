use std::f64::consts::PI;

use cairo::Context;

use crate::{connection::{Connection, ConnectionKind, self}, curve::Curve, lane::Lane, node::Node, road::Road};

/*
    Center Point
    Connections
    Lanes from / to connections

    2. Support for other modes!
    3. Signals / Priority

*/

pub struct Intersection {
    pub center: Node,
    pub connections: Vec<Connection>,
    pub lanes: Vec<Lane>,
}

impl Intersection {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            center: Node::new(x, y),
            connections: Vec::new(),
            lanes: Vec::new(),
        }
    }

    pub fn draw(&self, context: &Context) {
        for lane in &self.lanes {
            lane.draw(context);
        }
    }

    pub fn add_connection(&mut self, new_connection: Connection) {
        // Add connection to list
        self.connections.push(new_connection);

        self.lanes.clear();

        // Setup lanes for new connection:
        for connection in &self.connections {
            for second_connection in &self.connections {       
                if connection.kind == ConnectionKind::In && second_connection.kind == ConnectionKind::Out {
                    let curve = Curve::new(
                        connection.center,
                        second_connection.center,
                        connection.angle,
                        second_connection.angle,
                    );
                    self.lanes.push(Lane::new(curve, 10.0));
                }
            }
        }
    }
}
