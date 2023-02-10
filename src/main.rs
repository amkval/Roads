use std::{
    sync::{Arc, Mutex},
    thread,
};

use agent::Agent;
use cairo::glib::{Continue, MainContext, PRIORITY_DEFAULT};
use gtk4::{
    gdk::Key,
    prelude::{ApplicationExt, ApplicationExtManual, DrawingAreaExtManual},
    traits::{GestureExt, GestureSingleExt, GtkWindowExt, WidgetExt},
    Application, ApplicationWindow, DrawingArea,
};
use lane::{LaneKind, Lane};
use road::RoadKind;
use road_profile::RoadProfile;

extern crate cairo;

mod agent;
mod connection;
mod curve;
mod intersection;
mod lane;
mod map;
mod node;
mod property;
mod road;
mod road_profile;
mod toolbar;

use crate::intersection::Intersection;
use crate::map::Map;
use crate::road::Road;
use crate::toolbar::Toolbar;

const SCALE: f64 = 3.0;
const TILE: f64 = 8.0;

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
        let toolbar = Arc::new(Mutex::new(Toolbar::new()));
        let mut road_profiles: Vec<Arc<Mutex<RoadProfile>>> = Vec::new();

        // Add Basic Road Profiles
        {
            // One tile Car only
            let road_profile = Arc::new(Mutex::new(RoadProfile {
                right_lane_kinds: vec![LaneKind::Car],
                left_lane_kinds: vec![LaneKind::Car],
            }));
            road_profiles.push(road_profile);

            // Two tile Bike Lanes and Sidewalks
            let road_profile = Arc::new(Mutex::new(RoadProfile {
                right_lane_kinds: vec![LaneKind::Car, LaneKind::Bike, LaneKind::Pedestrian],
                left_lane_kinds: vec![LaneKind::Car, LaneKind::Bike, LaneKind::Pedestrian],
            }));
            road_profiles.push(road_profile);

            // Eight lanes
            let road_profile = Arc::new(Mutex::new(RoadProfile {
                right_lane_kinds: vec![LaneKind::Car, LaneKind::Car, LaneKind::Car, LaneKind::Car],
                left_lane_kinds: vec![LaneKind::Car, LaneKind::Car, LaneKind::Car, LaneKind::Car],
            }));
            road_profiles.push(road_profile);
        }

        // Set Draw Function
        {
            let map = map.clone();
            drawing_area.set_draw_func(move |_, context, _, _| match map.lock() {
                Ok(map) => {
                    context.scale(SCALE, SCALE);
                    context.set_line_width(1.0 / SCALE);
                    map.draw(context);
                }
                Err(_) => todo!(),
            });
        }

        // Mouse Click Handler
        {
            let map = map.clone();
            let toolbar = toolbar.clone();
            let gesture = gtk4::GestureClick::new();
            gesture.set_button(gtk4::gdk::ffi::GDK_BUTTON_PRIMARY as u32);
            gesture.connect_released(move |gesture: &gtk4::GestureClick, _, x, y| {
                gesture.set_state(gtk4::EventSequenceState::Claimed);
                println!("Mouse Button Released! {:.1} {:.1}", x, y);
                let new_x = (x / SCALE / TILE).round() * TILE;
                let new_y = (y / SCALE / TILE).round() * TILE;

                let mut map = map.lock().unwrap();
                let mut toolbar = toolbar.lock().unwrap();

                // Did we click on an existing Intersection?
                let result = map.intersections.iter().find(|intersection| {
                    (intersection.lock().unwrap().center.x - new_x).abs() < 10.0
                        && (intersection.lock().unwrap().center.y - new_y).abs() < 10.0
                });

                let new_intersection = match result {
                    Some(intersection) => intersection.clone(),
                    None => {
                        map.intersections
                            .push(Arc::new(Mutex::new(Intersection::new(new_x, new_y))));
                        map.intersections.last().unwrap().clone()
                    }
                };

                // Don't do anything if new == last.
                match &toolbar.selected {
                    Some(old) => {
                        if Arc::ptr_eq(&new_intersection, old) {
                            // They are the same.
                            return;
                        }
                    }
                    None => {}
                }

                if map.intersections.len() > 1 {
                    match &toolbar.selected {
                        Some(old_intersection) => {
                            let new_intersection_lock = new_intersection.lock().unwrap();
                            let old_intersection_lock = old_intersection.lock().unwrap();

                            // add road
                            let middle_intersection = Arc::new(Mutex::new(Intersection::new(
                                new_intersection_lock.center.x
                                    - (new_intersection_lock.center.x
                                        - old_intersection_lock.center.x)
                                        / 2.0,
                                new_intersection_lock.center.y
                                    - (new_intersection_lock.center.y
                                        - old_intersection_lock.center.y)
                                        / 2.0,
                            )));

                            drop(new_intersection_lock);
                            drop(old_intersection_lock);

                            let new_road = Arc::new(Mutex::new(Road::new(
                                old_intersection.clone(),
                                middle_intersection.clone(),
                                new_intersection.clone(),
                                road_profiles.last().unwrap().clone(),
                            )));

                            old_intersection
                                .lock()
                                .unwrap()
                                .roads
                                .push(new_road.clone());
                            middle_intersection
                                .lock()
                                .unwrap()
                                .roads
                                .push(new_road.clone());
                            new_intersection
                                .lock()
                                .unwrap()
                                .roads
                                .push(new_road.clone());

                            map.roads.push(new_road);
                        }
                        None => {}
                    }
                }

                toolbar.selected = Some(new_intersection.clone());
            });
            drawing_area.add_controller(&gesture);
        }

        // Button Release Handler
        {
            let map = map.clone();
            let event_controller = gtk4::EventControllerKey::new();
            event_controller.connect_key_released(move |_, key, _, _| match map.lock() {
                Ok(mut map) => match key {
                    Key::c => {
                        let agent = Arc::new(Mutex::new(Agent::new(
                            map.intersections
                                .first()
                                .unwrap()
                                .clone()
                                .lock()
                                .unwrap()
                                .lanes
                                .first()
                                .unwrap()
                                .clone(),
                            0.2,
                        )));
                        map.agents.push(agent);
                    }
                    _ => {}
                },
                Err(_) => todo!(),
            });

            window.add_controller(&event_controller);
        }

        window.set_child(Some(&drawing_area));
        window.show();

        // Update loop
        {
            let map = map.clone();
            let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);
            let _loop_thread = thread::spawn(move || {
                let duration = std::time::Duration::from_millis(10);
                loop {
                    let map = map.lock().unwrap();
                    for agent in &map.agents {
                        agent.lock().unwrap().update();
                    }
                    drop(map);
                    thread::sleep(duration);
                    sender.send(true).expect("Failed, blame the developer.");
                }
            });
            receiver.attach(None, move |_| {
                drawing_area.queue_draw();
                Continue(true)
            });
        }
    });
    app.run();
}
