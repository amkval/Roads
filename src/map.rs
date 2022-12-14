use cairo::Context;

use crate::{node::Node, road::Road};

pub struct Map {
    pub nodes: Vec<Node>,
    pub roads: Vec<Road>,
}

impl Map {
    pub fn new() -> Self {
        Self { nodes: Vec::new(), roads: Vec::new() }
    }

    pub fn draw(&self, context: &Context) {
        for road in &self.roads {
            road.draw(context);
        }

        for node in &self.nodes {
            node.draw(context, 30.0);
        }

        println!("Nodes: {}", self.nodes.len());
    }
}
