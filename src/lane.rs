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
        // Offset Curves
        context.set_source_rgb(0.20, 0.20, 0.20);
        let curve1 = self.curve.offset(self.width);
        let curve2 = self.curve.reverse().offset(self.width);

        curve1.plot(context);
        context.line_to(curve2.n0.x, curve2.n0.y);
        
        curve2.plot(context);
        context.line_to(curve1.n0.x, curve1.n0.y);
        
        context.set_source_rgb(0.50, 0.50, 0.50);
        context.stroke_preserve().expect("OMG!");
        context.fill().expect("Woops! Draw failed!");

        // Center Curve
        /*
        context.set_source_rgb(0.0, 0.0, 0.5);
        self.curve.n0.draw(context, 3.0);        
        self.curve.n1.draw(context, 3.0);
        
        context.set_source_rgb(0.0, 0.5, 0.0);
        self.curve.c.draw(context, 3.0);          
        */
        /*
        context.set_source_rgb(0.60, 0.20, 0.60);
        self.curve.plot(context);
        context.stroke().expect("Darn, you got me good!");
        */
    }
}
