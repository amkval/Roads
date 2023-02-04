use std::f64::consts::PI;

use cairo::Context;

use crate::node::Node;

pub struct Curve {
    pub n0: Node,
    pub n1: Node,
    pub n2: Node,
}

impl Curve {
    pub fn new(n0: Node, n1: Node, n2: Node) -> Self {
        Self { n0, n1, n2 }
    }

    pub fn reverse(&self) -> Curve {
        Curve::new( self.n2, self.n1, self.n0 )
    }

    // Fix this to make more sense
    pub fn offset(&self, offset: f64) -> Curve {
        let a0 = self.n0.get_angle(&self.n1) - PI / 2.;
        let a2 = self.n2.get_angle(&self.n1) + PI / 2.;
        
        let n0 = self.n0.offset(a0, offset);
        let n2 = self.n2.offset(a2, offset);

        let mut a = self.n0.get_angle(&self.n1);
        let b = self.n2.get_angle(&self.n1);

        if b < a {
            a += 2.0 * PI;
        }
        
        let c = a + (b - a) / 2.0;
        let middle_node = self.n1.offset(c, -offset);

        Curve::new(n0, middle_node, n2)
    }

    pub fn plot(&self, context: &Context) {
        quadratic_to(context, &self.n0, &self.n1, &self.n2);
    }

    pub fn plot_new(&self, context: &Context) {
        /*
        context.set_source_rgb(1.0, 0.0, 0.0);
        self.n0.draw(context, 3.0);
        context.set_source_rgb(0.0, 1.0, 0.0);
        self.n1.draw(context, 3.0);
        context.set_source_rgb(0.0, 0.0, 1.0);
        self.n2.draw(context, 3.0);
        */

        // Find angle to center of arc
        let mut a0 = self.n0.get_angle(&self.n1) - PI / 2.0;
        let a1 = self.n2.get_angle(&self.n1) + PI / 2.0;

        if a1 < a0 {
            a0 += 2.0 * PI;
        }

        // Make temporary offset points for calculations.
        let on0 = self.n0.offset(a0, 10.0);
        let on2 = self.n2.offset(a1, 10.0);

        // Fancy calculation to find intersection of lines
        let a1 = on0.y - self.n0.y;
        let b1 = self.n0.x - on0.x;
        let c1 = a1 * self.n0.x + b1 * self.n0.y;

        let a2 = on2.y - self.n2.y;
        let b2 = self.n2.x - on2.x;
        let c2 = a2 * self.n2.x + b2 * self.n2.y;

        let mut determinant = a1 * b2 - a2 * b1;
        determinant = (determinant * 100.0 ).round() / 100.0;

        if determinant.abs() == 0.0 {
            // The line is straight, draw a line
            context.move_to(self.n0.x, self.n0.y);
            context.line_to(self.n2.x, self.n2.y);
        } else {   
            // Find the new center
            let x = (b2 * c1 - b1 * c2) / determinant;
            let y = (a1 * c2 - a2 * c1) / determinant;
            let center = Node::new(x, y);

            // Find radius of new arc
            let radius = ((center.x - self.n0.x).powf(2.0) + (center.y - self.n0.y).powf(2.0)).sqrt();
            
            // Find start and stop of new arc
            let angle0 = center.get_angle(&self.n0);
            let angle2 = center.get_angle(&self.n2);
            
            // Left or right?
            let d = ((self.n0.x - self.n2.x).powf(2.0) + (self.n0.y - self.n2.y).powf(2.0)).sqrt();
            let d2 = ((on0.x - on2.x).powf(2.0) + (on0.y - on2.y).powf(2.0)).sqrt();


            if d < d2 {
                context.arc(center.x, center.y, radius, angle0, angle2);
            } else {
                context.arc_negative(center.x, center.y, radius, angle0, angle2);
            }
        }
    }

    pub fn length(&self) -> f64 {
        let dx = (self.n0.x - self.n2.x).abs();
        let dy = (self.n0.y - self.n2.y).abs();

        (dx.powi(2) + dy.powi(2)).sqrt()
    }
}

pub fn quadratic_to(context: &Context, n0: &Node, n1: &Node, n2: &Node) {
    context.curve_to(
        (n0.x + 2.0 * n1.x) / 3.0,
        (n0.y + 2.0 * n1.y) / 3.0,
        (n2.x + 2.0 * n1.x) / 3.0,
        (n2.y + 2.0 * n1.y) / 3.0,
        n2.x,
        n2.y,
    );
}
