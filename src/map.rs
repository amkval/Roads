use std::sync::{Arc, Mutex};

use cairo::Context;

use crate::{agent::Agent, intersection::Intersection, road::Road, TILE};

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
        context.set_source_rgb(0.36, 0.55, 0.35);
        context.paint().expect("omg!");

        // Draw temp grid, slow af
        context.set_source_rgb(0.55, 0.53, 0.58);
        
        for i in 0..96 {
            for j in 0..54 {
                context.move_to(0., (j as f64) * TILE + 4.0);
                context.line_to(1920., (j as f64) * TILE + 4.0);
                context.move_to((i as f64) * TILE + 4.0, 0.0);
                context.line_to((i as f64) * TILE + 4.0, 1080.0);
            }
        }
        context.stroke().expect("omg!");

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
