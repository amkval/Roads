use std::f64::consts::PI;

use cairo::Context;

use crate::{connection::{Connection, ConnectionKind}, curve::Curve, lane::Lane, node::Node, road::Road};

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
        self.center.draw(context, 3.0);

        for lane in &self.lanes {
            lane.draw(context);
        }
    }

    pub fn add_connection(&mut self, new_connection: Connection) {
        // Setup lanes for new connection:
        for connection in &self.connections {
            let mut a = connection.angle;
            let b = new_connection.angle;

            if b < a {
                a += 2.0 * PI;
            }
            
            let c = a + (b - a) / 2.0;
            let middle_node = self.center.offset(c, 10.0);
            let curve = Curve::new(
                connection.center,
                middle_node,
                new_connection.center,
            );
            self.lanes.push(Lane::new(curve, 10.0));
        }

        // Add road to list?
        self.connections.push(new_connection);
    }
}
