use std::f64::consts::PI;

use cairo::Context;

use crate::node::Node;

pub struct Curve {
    pub n0: Node,
    pub n1: Node,
    pub c: Node,
    pub is_curved: bool,
    pub is_reversed: bool,
}

impl Curve {
    pub fn new(n0: Node, n1: Node, a0: f64, a1: f64) -> Self {
        // Temporary Nodes for calculations
        let no0 = n0.offset(a0 - PI / 2.0, 10.0);
        let no1 = n1.offset(a1 + PI / 2.0, 10.0);

        // Find intersection of lines
        let a2 = no0.y - n0.y;
        let b2 = n0.x - no0.x;
        let c2 = a2 * n0.x + b2 * n0.y;

        let a3 = no1.y - n1.y;
        let b3 = n1.x - no1.x;
        let c3 = a3 * n1.x + b3 * n1.y;

        let mut determinant = a2 * b3 - a3 * b2;
        determinant = (determinant * 100.0).round() / 100.0;

        // The center of the arc
        if determinant.abs() == 0.0 {
            let d = n0.distance(&n1) / 2.0;
            let a  = n0.angle(&n1);
            let c = n0.offset(a, d);

            let mut a0t = a0;
            let mut a1t = a1;
            while a0t < 0.0 || a0t > PI * 2.0 {
                if a0t < 0.0 {
                    a0t += PI * 2.0;
                } else {
                    a0t -= PI * 2.0;
                }
            };

            while a1t < 0.0 || a1t > PI * 2.0 {
                if a1t < 0.0 {
                    a1t += PI * 2.0;
                } else {
                    a1t -= PI * 2.0;
                }
            };
            Self {
                n0,
                n1,
                c,
                is_curved: a0t == a1t,
                is_reversed: a0t == a1t,
            }
        } else {
            let x = (b3 * c2 - b2 * c3) / determinant;
            let y = (a2 * c3 - a3 * c2) / determinant;
            let c = Node::new(x, y);

            // Is the curve reversed? i.e draw from n1 to n0?
            let d0 = n0.distance(&c);
            let d1 = no0.distance(&c);
            let is_reversed = d1 > d0;
            Self {
                n0,
                n1,
                c,
                is_curved: true,
                is_reversed,
            }
        }
    }

    pub fn new_1(n0: Node, n1: Node, c: Node, is_curved: bool, is_reversed: bool) -> Self {
        Self {
            n0,
            n1,
            c,
            is_curved,
            is_reversed,
        }
    }

    pub fn reverse(&self) -> Curve {
        Curve::new_1(self.n1, self.n0, self.c, self.is_curved, !self.is_reversed)
    }

    pub fn offset(&self, mut offset: f64) -> Curve {
        if self.is_curved {
            if self.is_reversed {
                offset = -offset;
            }
            let a0 = self.n0.angle(&self.c);
            let a1 = self.n1.angle(&self.c);
            let n0 = self.n0.offset(a0, offset);
            let n1 = self.n1.offset(a1, offset);
            let c = self.c.clone();
            Curve::new_1(n0, n1, c, self.is_curved, self.is_reversed)
        } else {
            let a = self.n0.angle(&self.c) + PI / 2.0;
            let n0 = self.n0.offset(a, offset);
            let n1 = self.n1.offset(a, offset);
            let c = self.c.offset(a, offset);
            Curve::new_1(n0, n1, c, self.is_curved, self.is_reversed)
        }
    }

    pub fn plot(&self, context: &Context) {
        if !self.is_curved {
            // The line is straight, draw a line
            context.move_to(self.n0.x, self.n0.y);
            context.line_to(self.n1.x, self.n1.y);
        } else {
            // Find radius o arc
            let radius = self.c.distance(&self.n0);

            // Find start and stop angle of new arc
            let a0 = self.c.angle(&self.n0);
            let a1 = self.c.angle(&self.n1);

            if self.is_reversed {
                context.arc_negative(self.c.x, self.c.y, radius, a0, a1);
            } else {
                context.arc(self.c.x, self.c.y, radius, a0, a1);
            }
        }
    }

    pub fn length(&self) -> f64 {
        if self.is_curved {
            let mut a0 = self.c.angle(&self.n0);
            let mut a1 = self.c.angle(&self.n1);
            let radius = self.c.distance(&self.n0);
            // Find Delta Angle
            let da = {
                if self.is_reversed {
                    if a0 < a1 {
                        a0 += PI * 2.0;
                    }
                    a0 - a1
                } else {
                    if a1 < a0 {
                        a1 += PI * 2.0;
                    }
                    a1 - a0
                }
            };
            da * radius
        } else {
            let dx = (self.n0.x - self.n1.x).abs();
            let dy = (self.n0.y - self.n1.y).abs();   
            (dx.powi(2) + dy.powi(2)).sqrt()
        }
    }

    pub fn position_at(&self, d: f64) -> Node {
        if self.is_curved {
            let a0 = self.c.angle(&self.n0);
            let radius = self.c.distance(&self.n0);
            let a = d/radius;
            if self.is_reversed {
                self.c.offset(a0 - a, radius)
            } else {
                self.c.offset(a0 + a, radius)
            }
        } else {
            let d0 = self.n0.distance(&self.n1);
            let t = d / d0;
            Node::new((1.0 - t) * self.n0.x + t * self.n1.x, (1.0 - t) * self.n0.y + t * self.n1.y)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        let c0 = make_line();
        let c1 = c0.reverse();
        assert_eq!(c1.n0.x, c0.n1.x);
        assert_eq!(c1.n0.y, c0.n1.y);
        assert_eq!(c1.n1.x, c0.n0.x);
        assert_eq!(c1.n1.y, c0.n0.y);
        assert_ne!(c0.is_reversed, c1.is_reversed);
    }

    #[test]
    fn test_offset_line() {
        let c0 = make_line();
        let c1 = c0.offset(10.0);
        assert_eq!(c1.n0.x, 10.0);
        assert_eq!(c1.n0.y, 20.0);
        assert_eq!(c1.n1.x, 30.0);
        assert_eq!(c1.n1.y, 20.0);
        assert_eq!(c0.is_curved, c1.is_curved);
        assert_eq!(c0.is_reversed, c0.is_reversed);
    }

    #[test]
    fn test_length_line() {
        let c0 = make_line();
        let length = c0.length();
        assert_eq!(length, 20.0);
    }

    #[test]
    fn test_make_curve() {
        let c0 = make_curve();
        assert_eq!(c0.c.x.round(), 10.0);
        assert_eq!(c0.c.y.round(), 30.0);
        assert!(c0.is_curved);
    }

    #[test]
    fn test_length_curve() {
        let c0 = make_curve();
        let length = c0.length();
        assert!(!c0.is_reversed);
        assert_eq!((length * 100.0).round(), 3142.0);
    }

    #[test]
    fn test_length_curve_reverse() {
        let c0 = make_curve().reverse();
        let length = c0.length();
        assert_eq!(c0.c.x.round(), 10.0);
        assert_eq!(c0.c.y.round(), 30.0);
        assert!(c0.is_reversed);
        assert_eq!((length * 100.0).round(), 3142.0);
    }

    fn make_line() -> Curve {
        let n0 = Node::new(10.0, 10.0);
        let n1 = Node::new(30.0, 10.0);
        let a0 = 0.0;
        let a1 = PI;
        let c0 = Curve::new(n0, n1, a0, a1);
        c0
    }

    fn make_curve() -> Curve {
        let n0 = Node::new(10.0, 10.0);
        let n1 = Node::new(30.0, 30.0);
        let a0 = PI;
        let a1 = PI / 2.0;
        let c0 = Curve::new(n0, n1, a0, a1);
        c0
    }
}