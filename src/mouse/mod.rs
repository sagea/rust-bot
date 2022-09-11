use crossbeam::channel::{select, unbounded, Receiver};
use std::{ thread, time::{ Duration }, string:: { String }, sync:: {mpsc } };
use rand::Rng;
use enigo::*;

use crate::Vector::Rect;

pub fn click() {
  let mut rng = rand::thread_rng();
  let mut enigo = Enigo::new();
  let n1 = rng.gen_range(7..10);

  enigo.mouse_down(MouseButton::Left);
  thread::sleep(Duration::from_millis(n1));
  enigo.mouse_up(MouseButton::Left);
}

pub fn get_mouse_position() -> (i32, i32) {
  return Enigo::mouse_location();
}

// pub fn on_mouse_position_change() {
//   let mut last = Enigo::mouse_location();
//   output_mouse_location(last);
//   loop {
//     let mut cur = Enigo::mouse_location();
//       if last.0 != cur.0 || last.1 != cur.1 {
//           output_mouse_location(cur);
//           last = cur;
//       }
//       thread::sleep(Duration::from_millis(5));
//   }
// }

pub fn on_mouse_position_change() -> Receiver<(i32, i32)> {
  let (sender, receiver) = unbounded();
  thread::spawn(move || {
    let mut last = Enigo::mouse_location();
    sender.send(last).unwrap();
    loop {
      let mut cur = Enigo::mouse_location();
        if last.0 != cur.0 || last.1 != cur.1 {
          sender.send(last).unwrap();
          last = cur;
        }
        thread::sleep(Duration::from_millis(5));
    }
  });
  return receiver;
}


fn output_mouse_location((x, y): (i32, i32)) -> () {
  println!("{}:{}", x, y);
}

// pub fn mouse_enter(rect: Rect) -> (i32, i32) {
//   let last = get_mouse_position();
//   loop {
//     let pos = get_mouse_position();
//     if Rect::point_inside_tupl(pos) {
//       return pos
//     }
//   }
// }

// pub fn mouse_leave(rect: Rect) -> (i32, i32) {
//   let last = get_mouse_position();
//   loop {
//     let pos = get_mouse_position();
//     if !rect.point_inside_tupl(pos) {
//       return pos
//     }
//   }
// }

pub fn on_mouse_position_change_2() -> mpsc::Receiver<(i32, i32)> {
  let (tx, rx) = mpsc::channel();
  thread::spawn(move || {
    let mut last = get_mouse_position();
    tx.send(last).unwrap();
    loop {
      let cur = Enigo::mouse_location();
      if last.0 != cur.0 || last.1 != cur.1 {
          output_mouse_location(cur);
          last = cur;
      }
    }
  });
  return rx;
}
