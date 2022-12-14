use std::f64::consts::PI;

use cairo::Context;
use gtk4::{
    prelude::{ApplicationExt, ApplicationExtManual, DrawingAreaExtManual},
    traits::{GtkWindowExt, WidgetExt},
    Application, ApplicationWindow, DrawingArea,
};

extern crate cairo;

mod node;
mod curve;
mod lane;
mod road;

use crate::node::Node;
use crate::curve::Curve;
use crate::lane::Lane;
use crate::road::Road;



fn main() {
    let app = Application::builder()
        .application_id("dev.kval.roads")
        .build();

    app.connect_activate(|app| {
        if gtk4::init().is_err() {
            panic!("lmao");
        }
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(480)
            .default_height(360)
            .title("Hello, World!")
            .build();

        let drawing_area = DrawingArea::new();

        drawing_area.set_draw_func(move |_, context, _, _| {
            let locations: Vec<(f64, f64)> = vec![(100., 100.), (200., 100.), (300., 100.)];
            let nodes: Vec<Node> = locations.iter().map(|(x, y)| Node::new(*x, *y)).collect();
            let width = 20.;
            let curve = Curve::new(nodes[0], nodes[1], nodes[2]);
            let road = Road::new(curve, width);
            road.draw(context);
        });

        window.set_child(Some(&drawing_area));
        window.show();
    });
    app.run();
}







