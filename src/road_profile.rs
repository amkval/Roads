use crate::lane::LaneKind;

pub struct RoadProfile {
    pub right_lane_kinds: Vec<LaneKind>,
    pub left_lane_kinds: Vec<LaneKind>,
}

impl RoadProfile {
    pub fn new() -> Self {
        Self {
            right_lane_kinds: Vec::new(),
            left_lane_kinds: Vec::new(),
        }
    }

    pub fn width(&self) -> f64 {
        let mut width = 0.0;
        for lane in &self.right_lane_kinds {
            width += match lane {
                LaneKind::Car => 4.0,
                LaneKind::Bike => 2.0,
                LaneKind::Pedestrian => 2.0,
            };
        }
        for lane in &self.left_lane_kinds {
            width += match lane {
                LaneKind::Car => 4.0,
                LaneKind::Bike => 2.0,
                LaneKind::Pedestrian => 2.0,
            };
        }
        width
    }
}
