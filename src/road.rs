use crate::{lane::Lane, curve::Curve};
use cairo::Context;


pub struct Road {
    pub curve: Curve,
    pub width: f64,
}

impl Road {
    pub fn new(curve: Curve, width: f64) -> Self {
        Self { curve, width }
    }

    pub fn draw(&self, context: &Context) {
        self.curve.n0.draw(context, self.width);
        self.curve.n1.draw(context, self.width);
        self.curve.n2.draw(context, self.width);

        let lane1 = Lane::new(self.curve.offset(11.0), self.width/2.0);
        let lane2 = Lane::new(self.curve.reverse().offset(11.0), self.width/2.0);
        lane1.draw(context);
        lane2.draw(context);
    }
}