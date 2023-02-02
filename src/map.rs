use std::{rc::Rc, cell::RefCell};

use cairo::Context;

use crate::{road::Road, intersection::Intersection};

pub struct Map {
    pub intersections: Vec<Rc<RefCell<Intersection>>>,
    pub roads: Vec<Road>,
}

impl Map {
    pub fn new() -> Self {
        Self { intersections: Vec::new(), roads: Vec::new() }
    }

    pub fn draw(&self, context: &Context) {
        for road in &self.roads {
            road.draw(context);
        }

        for intersection in &self.intersections {
            intersection.borrow().draw(context);
        }
    }
}
