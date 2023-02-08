use std::{rc::Rc, cell::RefCell, sync::{Arc, Mutex}};

use cairo::Context;

use crate::intersection::Intersection;

pub struct Toolbar {
    // Alternatives
    // State
    pub selected: Option<Arc<Mutex<Intersection>>>,
}

impl Toolbar {
    pub fn new() -> Self {
        Self { selected: None}
    }

    pub fn draw(&self, context: &Context) {

    }
}