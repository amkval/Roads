use cairo::Context;

use crate::curve::Curve;

pub struct Lane {
    pub curve: Curve,
    pub width: f64,
}


impl Lane {
    pub fn new(curve: Curve, width: f64) -> Self {
        Self { curve, width }
    }

    pub fn draw(&self, context: &Context) {
        // Center Curve
        context.set_source_rgb(0.60, 0.20, 0.60);
        self.curve.plot_new(context);
        context.stroke().expect("Darn, you got me good!");

        // Offset Curves
        context.set_source_rgb(0.20, 0.20, 0.20);
        let curve1 = self.curve.offset(self.width);
        let curve2 = self.curve.reverse().offset(self.width);

        curve1.plot_new(context);
        context.line_to(curve2.n0.x, curve2.n0.y);
        
        curve2.plot_new(context);
        context.line_to(curve1.n0.x, curve1.n0.y);
        
        context.stroke_preserve().expect("OMG!");
        context.set_source_rgb(0.50, 0.50, 0.50);
        context.fill().expect("Woops! Draw failed!");
    }
}
