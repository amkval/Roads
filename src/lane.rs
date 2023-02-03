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
        self.curve.n0.draw(context, 2.0);
        self.curve.n1.draw(context, 2.0);
        self.curve.n2.draw(context, 2.0);

        // Center Curve
        context.set_source_rgb(0.60, 0.20, 0.60);
        self.curve.plot_new(context);
        context.stroke().expect("Darn, you got me good!");

        context.set_source_rgb(0.20, 0.60, 0.60);
        context.move_to(self.curve.n0.x, self.curve.n0.y);
        self.curve.plot(context);
        context.stroke().expect("Woops! Draw failed!");

        // Offset Curve
        context.set_source_rgb(0.20, 0.20, 0.20);
        let curve1 = self.curve.offset(self.width);
        context.move_to(curve1.n0.x, curve1.n0.y);
        curve1.plot(context);

        // Offset Curve Reverse
        context.set_source_rgb(0.20, 0.20, 0.60);
        let curve2 = self.curve.reverse().offset(self.width);
        context.line_to(curve2.n0.x, curve2.n0.y);
        curve2.plot(context);
        context.close_path();
        
        context.stroke().expect("OMG!");
        //context.fill().expect("Woops! Draw failed!");
    }
}
