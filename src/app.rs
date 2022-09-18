
use crate::mouse;
use crate::Vector;
use crate::inventory_manager;
use crate::util;
use crate::gui;

use Vector::{Rect};

use util::sleep;
use crossbeam::channel::{select, unbounded, Sender, Receiver};

use std::{thread};
use std::time::Duration;
use gui::{Logs};

#[derive(PartialEq)]
enum InEvents {
    Run,
    Stop,
}
#[derive(PartialEq)]
enum OutEvents {
    Started,
    Stopped,
    Completed,
}
struct SequenceThread {
    pub sender: Sender<InEvents>,
    pub receiver: Receiver<OutEvents>,
}

impl SequenceThread {
    pub fn new(actions: Vec<fn () -> ()>) -> Self {
        let (thread_sender, main_thread_receiver) = unbounded::<OutEvents>();
        let (main_thread_sender, thread_receiver) = unbounded::<InEvents>();
        thread::spawn(move || {
            let mut cycle = 0_i32;
            let mut active = false;
            loop {
                select! {
                    recv(thread_receiver) -> msg => {
                        let val = msg.unwrap();
                        if val == InEvents::Run {
                            cycle = 0;
                            active = true;
                            thread_sender.send(OutEvents::Started).unwrap();
                        } else if val == InEvents::Stop {
                            cycle = 0;
                            active = false;
                            thread_sender.send(OutEvents::Stopped).unwrap();
                        }
                    }
                    default(Duration::from_millis(50)) => {
                        if active {
                            if cycle as i32 >= actions.len() as i32 {
                                thread_sender.send(OutEvents::Completed).unwrap();
                                active = false
                            } else {
                                let method = actions[cycle as usize];
                                method();
                                cycle += 1;
                            }
                        }
                    }
                }
            }
        });
        SequenceThread {
            sender: main_thread_sender,
            receiver: main_thread_receiver,
        }
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
enum AppStatus {
    Running,
    Pausing,
    Paused,
}
pub struct App {
    runner: SequenceThread,
    mouse_pos: (i32, i32),
    menu_item_state: bool,
    run_count_total: i32,
    run_count_current: i32,
    is_point_in_bounds: bool,
    status: AppStatus,
}

impl App {
    pub fn new() -> Self {
        let mut actions: Vec<fn () -> ()> = Vec::new();
        actions.push(|| sleep(500, 1000));
        actions.push(mouse::click);
        actions.push(|| sleep(150, 300));
        actions.push(mouse::click);
        let runner = SequenceThread::new(actions);
        App {
            runner,
            mouse_pos: (0, 0),
            menu_item_state: false,
            run_count_total: 0,
            run_count_current: 0,
            is_point_in_bounds: false,
            status: AppStatus::Running,
        }
    }
    pub fn start(&mut self) {
        self.set_run_count_total(0);
        self.set_run_count_current(0);
        let magic_menu_status_change = inventory_manager::magic_menu_status_tracker();
        let mouse_pos_change = mouse::on_mouse_position_change();
        loop {
            select! {
                recv(self.runner.receiver) -> msg => {
                    let event = msg.unwrap();
                    if event == OutEvents::Stopped {
                        self.set_status(AppStatus::Paused);
                    } else if event == OutEvents::Started {
                        self.set_status(AppStatus::Running);
                    } else if event == OutEvents::Completed {
                        self.set_status(AppStatus::Paused);
                        self.increment_run_count();
                    }
                }
                recv(magic_menu_status_change) -> msg => {
                    self.set_menu_item_state(msg.unwrap());
                }
                recv(mouse_pos_change) -> msg => {
                    self.set_mouse_pos(msg.unwrap());
                }
            }
        }
    }
    pub fn increment_run_count(&mut self) {
        self.set_run_count_total(self.run_count_total + 1);
        self.set_run_count_current(self.run_count_current + 1);
    }
    pub fn set_run_count_total(&mut self, value: i32) {
        self.run_count_total = value;
        gui::set(Logs::RunCountTotal, format!("{}", self.run_count_total));
    }
    pub fn set_run_count_current(&mut self, value: i32) {
        self.run_count_current = value;
        gui::set(Logs::RunCountCurrent, format!("{}", self.run_count_current));
    }
    pub fn set_mouse_pos(&mut self, pos: (i32, i32)) {
        self.mouse_pos = pos;
        let cast_hla_button = Rect::from_points(1486, 383, 1509, 405);
        let item_slot_button = Rect::from_points(1493, 386, 1526, 418);
        let shared_cast_hla_and_item_slot = Rect::overlap(cast_hla_button, item_slot_button).expect("The cast hla button and item slot button should overlap.");
        self.is_point_in_bounds = Rect::point_inside_tupl(&shared_cast_hla_and_item_slot, self.mouse_pos);
        gui::set(Logs::PointInBounds, format!("{}", self.is_point_in_bounds));
        gui::set(Logs::MousePosition, format!("x: {}, y: {}", self.mouse_pos.0, self.mouse_pos.1));
        self.attempt_run();
    }
    pub fn set_menu_item_state(&mut self, active: bool) {
        self.menu_item_state = active;
        gui::set(Logs::MagicMenuFocused, format!("{}", self.menu_item_state));
        self.attempt_run();
    }
    pub fn set_status(&mut self, status: AppStatus) {
        self.status = status;
        gui::set(Logs::AppStatus, format!("{:?}", status));
        self.attempt_run();
    }
    pub fn attempt_run(&mut self) {
        if self.status == AppStatus::Paused && self.is_point_in_bounds && self.menu_item_state {
            self.runner.sender.send(InEvents::Run).unwrap();
            self.set_status(AppStatus::Running);
        } else if self.status == AppStatus::Running && !self.is_point_in_bounds {
            self.runner.sender.send(InEvents::Stop).unwrap();
            self.set_status(AppStatus::Pausing);
            self.set_run_count_current(0);
        } else if self.status == AppStatus::Paused && !self.is_point_in_bounds {
            self.set_run_count_current(0);
        }
    }
}
