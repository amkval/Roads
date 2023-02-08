use std::sync::{Arc, Mutex};

use cairo::Context;

use crate::{agent::Agent, intersection::Intersection, road::Road};

pub struct Map {
    pub intersections: Vec<Arc<Mutex<Intersection>>>,
    pub roads: Vec<Arc<Mutex<Road>>>,
    pub agents: Vec<Arc<Mutex<Agent>>>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            intersections: Vec::new(),
            roads: Vec::new(),
            agents: Vec::new(),
        }
    }

    pub fn draw(&self, context: &Context) {
        for road in &self.roads {
            road.lock().unwrap().draw(context);
        }

        for intersection in &self.intersections {
            intersection.lock().unwrap().draw(context);
        }

        for agent in &self.agents {
            agent.lock().unwrap().draw(context);
        }
    }
}
