use std::{sync::{Arc, Mutex}, f64::consts::PI};
use rand::{distributions::Uniform, prelude::Distribution};

use cairo::Context;

use crate::{node::Node, lane::Lane};

pub struct Agent {
    pub c: Node,
    pub l: Arc<Mutex<Lane>>,
    pub distance: f64,
}

impl Agent {

    pub fn new(l: Arc<Mutex<Lane>>, distance: f64) -> Self {
        let c = l.lock().unwrap().position_at(distance);
        Self { c, l, distance }
    }

    pub fn update(& mut self) {
        let mut distance_to_move = 2.0;
        let mut moved_distance = 0.0;
        while distance_to_move > moved_distance {
            let lane_length = self.l.lock().unwrap().length();
            let remaining_distance = lane_length - self.distance;
            if remaining_distance < distance_to_move - moved_distance {
                moved_distance += remaining_distance;
                distance_to_move -= remaining_distance;
                let mut new_lane_number = self.l.lock().unwrap().c1.lock().unwrap().out_lane.len();
                let mut rng = rand::thread_rng();
                new_lane_number = Uniform::from(0..new_lane_number).sample(&mut rng);
                let new_lane = self.l.lock().unwrap().c1.lock().unwrap().out_lane[new_lane_number].clone();
                self.distance = 0.0;
                self.l = new_lane;
            } else {
                moved_distance += distance_to_move;
                distance_to_move -= moved_distance;
            }
        }
        self.distance += moved_distance;
        let new_position = self.l.lock().unwrap().position_at(self.distance);
        self.c = new_position;
    }

    pub fn draw(&self, context: &Context ) {
        context.set_source_rgb(1.0, 0.0, 0.0);
        context.arc(self.c.x, self.c.y, 4.0, 0.0, PI * 2.0);
        context.fill().expect("Woops! Draw failed!");
    }
}