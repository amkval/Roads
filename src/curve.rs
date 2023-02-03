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

    pub fn offset(&self, offset: f64) -> Curve {
        let a0 = self.n0.get_angle(&self.n1) - PI / 2.;
        let a1 = self.n1.get_angle(&self.n2) - PI / 2.;
        let a2 = self.n2.get_angle(&self.n1) + PI / 2.;
        println!("a0: {:.2}", a0);
        println!("a1: {:.2}", a1);
        println!("a2: {:.2}", a2);
        let n0 = self.n0.offset(a0, offset);
        println!("n0: {:.0},{:.0}", n0.x, n0.y);
        println!("self.n0: {:.0},{:.0}", self.n0.x, self.n0.y);
        let n1 = self.n1.offset(a1, offset);
        println!("n1: {:.0},{:.0}", n1.x, n1.y);
        println!("self.n1: {:.0},{:.0}", self.n1.x, self.n1.y);
        let n2 = self.n2.offset(a2, offset);
        println!("n2: {:.0},{:.0}", n2.x, n2.y);
        println!("self.n2: {:.0},{:.0}", self.n2.x, self.n2.y);
        Curve::new(n0, n1, n2)
    }

    pub fn plot(&self, context: &Context) {
        quadratic_to(context, &self.n0, &self.n1, &self.n2);
    }

    pub fn plot_new(&self, context: &Context) {
        // New plotting function using arcs

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
        println!("Determinant {:.5}", determinant);

        determinant = (determinant * 100.0 ).round() / 100.0;

        println!("Determinant {:.5}", determinant);

        if determinant.abs() == 0.0 {
            // The line is straight, draw a line
            context.move_to(self.n0.x, self.n0.y);
            context.line_to(self.n2.x, self.n2.y);
        } else {   
            // Find the new center
            let x = (b2 * c1 - b1 * c2) / determinant;
            let y = (a1 * c2 - a2 * c1) / determinant;
            let center = Node::new(x, y);

            center.draw(context, 5.0);
            self.n0.draw(context, 7.0);
            self.n2.draw(context, 5.0);

            // Find radius of new arc
            let radius = ((center.x - self.n0.x).powf(2.0) + (center.y - self.n0.y).powf(2.0)).sqrt();
            
            // Find start and stop of new arc
            let mut angle0 = center.get_angle(&self.n0);
            let mut angle2 = center.get_angle(&self.n2);
            
            // Left or right?
            let d = ((self.n0.x - self.n2.x).powf(2.0) + (self.n0.y - self.n2.y).powf(2.0)).sqrt();
            let d2 = ((on0.x - on2.x).powf(2.0) + (on0.y - on2.y).powf(2.0)).sqrt();


            if d < d2 {
                context.arc(center.x, center.y, radius, angle0, angle2);
            } else {
                context.arc(center.x, center.y, radius, angle2, angle0);
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
