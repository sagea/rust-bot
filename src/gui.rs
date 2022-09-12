use std::str;
use lazy_static::lazy_static;

use clearscreen;
use std::collections::HashMap;
use std::sync::Mutex;
use termion::{color, style};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Logs {
  RunCountTotal,//: &'static str = "Run Count (Total)";
  RunCountCurrent,//: &'static str = "Run Count (Current)";
  PointInBounds,//: &'static str = "Point In Bounds";
  MousePosition,//: &'static str = "Mouse Position";
  MagicMenuFocused,//: &'static str = "Magic Menu Focused";
  AppStatus,//: &'static str = "App Status";
}
lazy_static! {
  static ref STATE: Mutex<HashMap<Logs, String>> = {
      let mut map = HashMap::new();
      map.insert(Logs::RunCountTotal, String::from(""));
      map.insert(Logs::RunCountCurrent, String::from(""));
      map.insert(Logs::AppStatus, String::from(""));
      map.insert(Logs::MousePosition, String::from(""));
      map.insert(Logs::PointInBounds, String::from(""));
      map.insert(Logs::MagicMenuFocused, String::from(""));
      Mutex::new(map)
  };
}

pub fn set(key: Logs, value: String) {
  thread::spawn(move || {
    let mut map = STATE.lock().unwrap();
    let og = map.get(&key);
    if og.is_some() && og.unwrap() == &value {
      // don't update if the value is the same
      return ()
    }
    map.insert(key, value);
    clearscreen::clear().expect("failed to clear screen");
    for (key, value) in map.iter() {
      println!("{}{:?}{}: {}", color::Fg(color::Red), key.clone(), style::Reset, value);
    }
  });
}
