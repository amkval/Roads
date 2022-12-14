use std::{
    sync::{Arc, Mutex},
    thread,
};

use cairo::glib::{Continue, MainContext, PRIORITY_DEFAULT};
use gtk4::{
    prelude::{ApplicationExt, ApplicationExtManual, DrawingAreaExtManual},
    traits::{GestureExt, GestureSingleExt, GtkWindowExt, WidgetExt},
    Application, ApplicationWindow, DrawingArea,
};

extern crate cairo;

mod curve;
mod lane;
mod map;
mod node;
mod road;

use crate::curve::Curve;
use crate::map::Map;
use crate::node::Node;
use crate::road::Road;

fn main() {
    let app = Application::builder()
        .application_id("dev.kval.roads")
        .build();

    app.connect_activate(|app| {
        if gtk4::init().is_err() {
            panic!("gtk4 failed, blame the developer!");
        }
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(480)
            .default_height(360)
            .title("Hello, World!")
            .build();

        let drawing_area = DrawingArea::new();
        let map = Arc::new(Mutex::new(Map::new()));

        // Initialize Game
        {
            let map = map.clone();
            let locations: Vec<(f64, f64)> = vec![(100., 100.), (200., 100.), (300., 100.)];
            let nodes: Vec<Node> = locations.iter().map(|(x, y)| Node::new(*x, *y)).collect();
            let width = 20.;
            let curve = Curve::new(nodes[0], nodes[1], nodes[2]);
            let road = Road::new(curve, width);
            let mut map = map.lock().unwrap();
            map.roads.push(road);
        }

        // Set Draw Function
        {
            let map = map.clone();
            drawing_area.set_draw_func(move |_, context, _, _| {
                let map = map.lock().unwrap();
                map.draw(context);
            });
        }

        // Mouse Click Handler
        {
            let map = map.clone();
            let gesture = gtk4::GestureClick::new();
            gesture.set_button(gtk4::gdk::ffi::GDK_BUTTON_PRIMARY as u32);
            gesture.connect_released(move |gesture: &gtk4::GestureClick, _, x, y| {
                gesture.set_state(gtk4::EventSequenceState::Claimed);
                println!("Mouse Button Released! {:.1} {:.1}", x, y);
                map.lock().unwrap().nodes.push(Node::new(x, y));
            });
            drawing_area.add_controller(&gesture);
        }

        window.set_child(Some(&drawing_area));
        window.show();

        // Update loop
        let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);
        let loop_thread = thread::spawn(move || {
            let duration = std::time::Duration::from_millis(60);
            loop {
                thread::sleep(duration);
                println!("Loop! Woop!");
                sender.send(true).expect("Oh no! Update failed!");
            }
        });
        receiver.attach(None, move |_| {
            drawing_area.queue_draw();
            Continue(true)
        });
    });
    app.run();
}
