
use crossbeam::channel::{unbounded, Receiver};

use crate::screen;
use crate::Vector;

// use std::sync::mpsc::{self, Receiver};
use std::thread;

#[derive(Debug, Clone, Copy)]

pub enum MenuItemTrackerState {
  Active,
  Inactive,
}

fn is_slot_active(pixels: Vec<u8>) -> bool {
  let mut ba: i32 = 0;
  let mut ga: i32 = 0;
  let mut ra: i32 = 0;
  let px_count = (pixels.len() / 4) as i32; 
  for i in (0..pixels.len()).step_by(4) {
    ba += pixels[i] as i32;
    ga += pixels[i + 1] as i32;
    ra += pixels[i + 2] as i32;
  }
  let b = ba / px_count;
  let g = ga / px_count;
  let r = ra / px_count;
  return (r as f32 / (b + g + r) as f32) > 0.5;
}

pub fn magic_menu_status_tracker() -> Receiver<bool> {
  use crate::util::sleep_exact;
  let (sender, receiver) = unbounded();
  thread::spawn(move || {
    let rect = Vector::Rect::from_size(1624, 240, 5, 5);
    let mut last_state = 0 as u8;
    loop {
      let screenshot = screen::screenshot(rect.x1, rect.y1, rect.width, rect.height);
      if let Ok(image) = screenshot {
        let mut cur_state = 0;
        if is_slot_active(image.pixels) {
          cur_state = 1;
        } else {
          cur_state = 2;
        }
        if last_state != cur_state {
          last_state = cur_state;
          if cur_state == 1 {
            sender.send(true).unwrap();
          } else if cur_state == 2 {
            sender.send(false).unwrap();
          }
        }
      } else {
        println!("Error imageError");
      }
      sleep_exact(50)
    }
  });

  return receiver;
}