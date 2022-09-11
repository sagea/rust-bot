mod screen;
mod mouse;
mod Vector;
mod inventory_manager;
mod util;

use Vector::{Rect};
use util::sleep;
use crossbeam::channel::{select, unbounded, Sender};
use std::thread;


struct ChangeResult (bool, bool);

struct ActionSequence {}
fn go_og () {

    let magic_menu_button = Rect::from_points(1620, 238, 1653, 268);
    let cast_hla_button = Rect::from_points(1453, 387, 1475, 404);
    let item_slot_button = Rect::from_points(1458, 388, 1488, 418);
    let shared_cast_hla_and_item_slot = Rect::overlap(cast_hla_button, item_slot_button).expect("The cast hla button and item slot button should overlap.");
    let mut pointInside = false;
    fn shouldStop(rect: &Rect) -> bool {
        let mouse_position = mouse::get_mouse_position();
        let res = Rect::point_inside_tupl(&rect, mouse_position);
        res == false
    }
    let mut initActions: Vec<fn() -> ()> = Vec::new();
    initActions.push(|| {println!("Starting in 2");});
    initActions.push(|| {sleep(1000, 1001)});
    initActions.push(|| {println!("Starting in 1");});
    initActions.push(|| {sleep(1000, 1001)});
    initActions.push(|| {println!("Starting in 0");});
    initActions.push(|| {sleep(1000, 1001)});

    let mut eachNotInit: Vec<fn() -> ()> = Vec::new();
    eachNotInit.push(|| sleep(3200, 4000));

    let mut loopActions: Vec<fn() -> ()> = Vec::new();
    loopActions.push(|| { mouse::click(); });
    loopActions.push(|| sleep(150, 300));
    loopActions.push(|| mouse::click());
    let mut total = 0;
    let mut total_round = 0;
    
    loop {
        let mouse_position = mouse::get_mouse_position();
        let mut changed = false;
        if Rect::point_inside_tupl(&shared_cast_hla_and_item_slot, mouse_position) {
            if pointInside == false {
                pointInside = true;
                changed = true;
            }
        } else {
            if pointInside == true {
                pointInside = false;
                changed = true;
            }
        }
        if pointInside {
            let mut actions: Vec<fn() -> ()> = Vec::new();
            if changed {
                total_round = 0;
                println!("changed");
                actions.append(&mut initActions.clone());
            } else {
                actions.append(&mut eachNotInit.clone());
            }
            actions.append(&mut loopActions.clone());
            for m in &actions {
                if shouldStop(&shared_cast_hla_and_item_slot) {
                    println!("Stopping");
                    break;
                }
                m();
            }
            total += 1;
            total_round += 1;
            println!("Total: {total} - Current: {total_round}");
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct GoMessage {
    magic_menu_active: bool,
    mouse_pos: (i32, i32),
}

fn go () -> Sender<GoMessage> {
    let (sender, receiver) = unbounded::<GoMessage>();
    thread::spawn(move || {
        let magic_menu_button = Rect::from_points(1620, 238, 1653, 268);
        let cast_hla_button = Rect::from_points(1453, 387, 1475, 404);
        let item_slot_button = Rect::from_points(1458, 388, 1488, 418);
        let shared_cast_hla_and_item_slot = Rect::overlap(cast_hla_button, item_slot_button).expect("The cast hla button and item slot button should overlap.");
        let mut pointInside = false;
        fn shouldStop(rect: &Rect) -> bool {
            let mouse_position = mouse::get_mouse_position();
            let res = Rect::point_inside_tupl(&rect, mouse_position);
            res == false
        }
        let mut initActions: Vec<fn() -> ()> = Vec::new();
        initActions.push(|| {println!("Starting in 2");});
        initActions.push(|| {sleep(1000, 1001)});
        initActions.push(|| {println!("Starting in 1");});
        initActions.push(|| {sleep(1000, 1001)});
        initActions.push(|| {println!("Starting in 0");});
        initActions.push(|| {sleep(1000, 1001)});

        let mut eachNotInit: Vec<fn() -> ()> = Vec::new();
        eachNotInit.push(|| sleep(3200, 4000));

        let mut loopActions: Vec<fn() -> ()> = Vec::new();
        loopActions.push(|| { mouse::click(); });
        loopActions.push(|| sleep(150, 300));
        loopActions.push(|| mouse::click());
        let mut total = 0;
        let mut total_round = 0;
        let mut mouse_position = (0, 0);
        let mut magic_menu_active = false;
        let mut menu_changed_since_run = false;
        let mut round_active = false;
        
        loop {
            select! {
                recv(receiver) -> msg => {
                    match msg {
                        Ok(go_message) => {
                            if magic_menu_active != go_message.magic_menu_active {
                                menu_changed_since_run = true;
                                magic_menu_active = go_message.magic_menu_active;
                            }
                            mouse_position = (go_message.mouse_pos.0, go_message.mouse_pos.1);
                            let point_inside = Rect::point_inside_tupl(&shared_cast_hla_and_item_slot, mouse_position);
                            if point_inside && menu_changed_since_run && magic_menu_active == true {
                                menu_changed_since_run = false;
                                sleep(1000, 1500);
                                let mut actions: Vec<fn() -> ()> = Vec::new();
                                // if changed {
                                //     total_round = 0;
                                //     println!("changed");
                                //     actions.append(&mut initActions.clone());
                                // } else {
                                //     actions.append(&mut eachNotInit.clone());
                                // }
                                // actions.append(&mut eachNotInit.clone());
                                actions.append(&mut loopActions.clone());
                                for m in &actions {
                                    // if shouldStop(&shared_cast_hla_and_item_slot) {
                                    //     println!("Stopping");
                                    //     break;
                                    // }
                                    m();
                                }
                                total += 1;
                                total_round += 1;
                                println!("Total: {total} - Current: {total_round}");
                            }
                            // mouse_position = go_message.mouse_pos;
                            // magic_menu_active = go_message.magic_menu_active;
                        },
                        Err(_) => {}
                    }
                }
            };
            
        }
    });
    sender
}

fn main() {
    let iv = inventory_manager::magic_menu_status_tracker();
    let mouse_pos_change = mouse::on_mouse_position_change();
    let runner = go();
    let mut mouse_pos: (i32, i32) = (0, 0);
    let mut magic_menu_active = false;
    loop {
        select! {
            recv(iv) -> msg => {
                let r = msg.unwrap();
                match msg {
                    Ok(inventory_manager::MenuItemTrackerState::active) => {
                        magic_menu_active = true;
                        runner.send(GoMessage {
                            magic_menu_active,
                            mouse_pos,
                        }).unwrap();
                    },
                    Ok(inventory_manager::MenuItemTrackerState::inactive) => {
                        magic_menu_active = false;
                        runner.send(GoMessage {
                            magic_menu_active,
                            mouse_pos,
                        }).unwrap();
                    },
                    Err(_) => panic!("Somehow there's an error"),
                }
                println!("Magic Active: {:?}", r);
            },
            recv(mouse_pos_change) -> msg => {
                let pos = msg.unwrap();
                mouse_pos = pos;
                runner.send(GoMessage {
                    magic_menu_active,
                    mouse_pos,
                }).unwrap();
            }
        }
    }
}
