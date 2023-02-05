use std::{
    sync::{Arc, Mutex},
    thread, rc::Rc, cell::RefCell, f64::consts::PI,
};

use cairo::glib::{Continue, MainContext, PRIORITY_DEFAULT};
use gtk4::{
    prelude::{ApplicationExt, ApplicationExtManual, DrawingAreaExtManual},
    traits::{GestureExt, GestureSingleExt, GtkWindowExt, WidgetExt},
    Application, ApplicationWindow, DrawingArea,
};

extern crate cairo;

mod connection;
mod curve;
mod intersection;
mod lane;
mod map;
mod node;
mod property;
mod road;

use crate::{intersection::Intersection, node::Node, curve::Curve};
use crate::map::Map;
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

                let mut map = map.lock().unwrap();

                // Did we click on an existing Intersection?
                let result = map.intersections.iter().find(|intersection| {
                    (intersection.borrow().center.x - x).abs() < 20.0
                        && (intersection.borrow().center.y - y).abs() < 20.0
                });

                let new_intersection = match result {
                    Some(intersection) => intersection.clone(),
                    None => {
                        map.intersections.push(Rc::new(RefCell::new(Intersection::new(x, y))));
                        map.intersections.last().unwrap().clone()
                    }
                };

                // Add Road if we have more than one Intersection.
                if map.intersections.len() > 1 {
                    let old_intersection= map.intersections.get(map.intersections.len()-2).unwrap().clone();

                    let middle_intersection = Rc::new(RefCell::new(Intersection::new(
                        new_intersection.borrow().center.x
                            - (new_intersection.borrow().center.x - old_intersection.borrow().center.x) / 2.0,
                        new_intersection.borrow().center.y
                            - (new_intersection.borrow().center.y - old_intersection.borrow().center.y) / 2.0,
                    )));

                    let new_road = Road::new(
                        old_intersection,
                        middle_intersection,
                        new_intersection,
                        20.0,
                    );

                    map.roads.push(new_road);
                }
            });
            drawing_area.add_controller(&gesture);
        }

        window.set_child(Some(&drawing_area));
        window.show();

        // Update loop
        let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);
        let _loop_thread = thread::spawn(move || {
            let duration = std::time::Duration::from_millis(60);
            loop {
                thread::sleep(duration);
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
