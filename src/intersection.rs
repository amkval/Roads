use std::{
    f64::consts::PI,
    sync::{Arc, Mutex},
};

use cairo::Context;

use crate::{
    connection::{Connection, ConnectionKind},
    curve::Curve,
    lane::Lane,
    node::Node,
    road::Road,
};

pub struct Intersection {
    pub center: Node,
    pub roads: Vec<Arc<Mutex<Road>>>,
    pub connections: Vec<Arc<Mutex<Connection>>>,
    pub lanes: Vec<Arc<Mutex<Lane>>>,
}

impl Intersection {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            center: Node::new(x, y),
            roads: Vec::new(),
            connections: Vec::new(),
            lanes: Vec::new(),
        }
    }

    pub fn draw(&self, context: &Context) {
        for lane in &self.lanes {
            lane.lock().unwrap().draw(context);
        }
    }

    pub fn get_connections(&mut self, a: f64, width: f64) -> Vec<Arc<Mutex<Connection>>> {
        let mut cs: Vec<Arc<Mutex<Connection>>> = Vec::new();
        
        let a0 = a - PI / 2.0;
        let a1 = a + PI / 2.0;
        
        // Connection centers
        let n0 = self.center.offset(a, width * 1.5).offset(a0, width / 2.0);
        let n2 = self.center.offset(a, width * 1.5).offset(a1, width / 2.0);
        
        // New connections
        let c0 = Arc::new(Mutex::new(Connection::new(
            n0,
            ConnectionKind::Out,
            a,
            width / 2.,
        )));
        let c1 = Arc::new(Mutex::new(Connection::new(
            n2,
            ConnectionKind::In,
            a,
            width / 2.,
        )));
        
        cs.push(c0.clone());
        cs.push(c1.clone());
        
        self.connections.push(c0.clone());
        self.connections.push(c1.clone());
        
        println!("Bananaramalamadingdong!");
        self.add_lanes();
        println!("we are here???");
        
        cs
    }

    pub fn add_lanes(&mut self) {
        println!("Wombocombo");
        self.lanes.clear();
        println!("Catchem");
        // Setup lanes for new connection:
        for c0 in &self.connections {
            println!("we are here?");
            for c1 in &self.connections {
                println!("Bananarama");
                if !Arc::ptr_eq(c0, c1) {
                    let mut c0_lock = c0.lock().unwrap();
                    let mut c1_lock = c1.lock().unwrap();
                    let c0k = &c0_lock.kind;
                    let c1k = &c1_lock.kind;
                    
                    println!("Nandene?");
                    
                    if *c0k == ConnectionKind::In && *c1k == ConnectionKind::Out {
                        
                        let curve = Curve::new(
                            c0_lock.center,
                            c1_lock.center,
                            c0_lock.angle,
                            c1_lock.angle,
                        );

                        let l = Arc::new(Mutex::new(Lane::new(
                            c0.clone(),
                            c1.clone(),
                            curve,
                            10.0,
                        )));

                        c0_lock.out_lane.push(l.clone());
                        c1_lock.in_lane.push(l.clone());
                        
                        //drop(c0_lock);
                        //drop(c1_lock);
                        
                        self.lanes.push(l);
                    }
                }
            }
        }
    }
}
