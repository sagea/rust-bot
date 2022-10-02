use crossbeam::channel::{unbounded, Receiver};
use std::{ thread, time::{ Duration }, sync:: {mpsc } };
use rand::Rng;
use enigo::*;

pub fn click() {
  let mut rng = rand::thread_rng();
  let mut enigo = Enigo::new();
  let n1 = rng.gen_range(7..10);

  enigo.mouse_down(MouseButton::Left);
  thread::sleep(Duration::from_millis(n1));
  enigo.mouse_up(MouseButton::Left);
}

pub fn _get_mouse_position() -> (i32, i32) {
  Enigo::mouse_location()
}

pub fn on_mouse_position_change() -> Receiver<(i32, i32)> {
  let (sender, receiver) = unbounded();
  thread::spawn(move || {
    let mut last = Enigo::mouse_location();
    sender.send(last).unwrap();
    loop {
      let cur = Enigo::mouse_location();
        if last.0 != cur.0 || last.1 != cur.1 {
          sender.send(last).unwrap();
          last = cur;
        }
        thread::sleep(Duration::from_millis(5));
    }
  });
  receiver
}

fn output_mouse_location((x, y): (i32, i32)) {
  println!("{}:{}", x, y);
}

pub fn on_mouse_position_change_2() -> mpsc::Receiver<(i32, i32)> {
  let (tx, rx) = mpsc::channel();
  thread::spawn(move || {
    let mut last = _get_mouse_position();
    tx.send(last).unwrap();
    loop {
      let cur = Enigo::mouse_location();
      if last.0 != cur.0 || last.1 != cur.1 {
          output_mouse_location(cur);
          last = cur;
      }
    }
  });
  rx
}

use tokio::task;
use tokio::time;

use crate::vector::Rect;

pub async fn when_mouse_enters(rect: &Rect) -> () {
  let r = rect.clone();
  let join = task::spawn(async move {
    loop {
      let cur = Enigo::mouse_location();
      if Rect::point_inside_tupl(&r, cur) {
        return;
      }
      time::sleep(time::Duration::from_millis(5)).await;
    }
  });
  join.await.unwrap();
  return ();
}