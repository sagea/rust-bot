
use std::io::{Write, stdout};
use std::thread;
use std::time::Duration;
use lazy_static::lazy_static;
use std::sync::{Arc};
use std::sync::Mutex;
use termion::{color, style};
use std::collections::BTreeMap;

static ROOT_GROUP: &str = "$$static$$";

lazy_static! {
  static ref STATE: Arc<Mutex<BTreeMap<&'static str, Arc<Mutex<BTreeMap<&'static str, String>>>>>> = {
    let mut map = BTreeMap::new();
    map.insert(ROOT_GROUP, Arc::new(Mutex::new(BTreeMap::new())));
    Arc::new(Mutex::new(map))
  };
}

pub struct Logs {}
impl Logs {
  pub const RunCountTotal: &'static str = "Run count (total)";
  pub const RunCountCurrent: &'static str = "Run count (current)";
  pub const AppStatus: &'static str = "App status";
  pub const MousePosition: &'static str = "Mouse pos";
  pub const PointInBounds: &'static str = "Mouse in position";
  pub const MagicMenuFocused: &'static str = "Magic menu open";
}

pub fn start() {
  thread::spawn(|| {
    loop {
      render();
      thread::sleep(Duration::from_millis(30));
    }
  });
}

pub fn render() {
  use termion::color::*;
  use termion::clear;
  use termion::cursor::{Goto, Show};
  let state = STATE.lock().unwrap();
  let mut stdout = stdout();
  write!(stdout, "{}", clear::All).unwrap();
  let mut offsety = 5;
  let test_offset = 5;
  let change = 1;
  for (group_name, group) in state.iter() {
    let is_root_group = group_name.to_string().as_str() == "$$static$$";
    let _header_offset_y: u16 = if is_root_group { 0 } else { 1 };
    let offsetx: u16 = if is_root_group { 0 } else { 3 };
    if !is_root_group {
      offsety += 1;
      write!(stdout, "{}{}", Goto(0, offsety), Show).unwrap();
      write!(stdout, "{}{}{}", Fg(Red), group_name, style::Reset).unwrap();
      write!(stdout, "{}", clear::UntilNewline).unwrap();
      write!(stdout, "{}", Goto(0, offsety + 1)).unwrap();
    }
  let log_group = Arc::clone(group);
  let f = log_group.lock().unwrap();
    write!(stdout, "{} val {}", Goto(change, 2), test_offset).unwrap();
    for (prop, value) in f.iter().rev() {
      offsety += 1;
      write!(stdout, "{}{}", Goto(offsetx, offsety), Show).unwrap();
      write!(stdout, "{}", clear::UntilNewline).unwrap();
      write!(stdout, "{}{}{}: {}", color::Fg(color::Red), prop, style::Reset, value).unwrap();
      write!(stdout, "{}{} expected:{} actual: {}", Goto(1, 3), clear::UntilNewline, offsety, test_offset).unwrap();
    }
  }
  write!(stdout, "{}", Goto(1, offsety + 1)).unwrap();
  write!(stdout, "{}", clear::AfterCursor).unwrap();
  stdout.flush().unwrap();
}

pub fn set_group(group_name: &'static str, key: &'static str, value: String) {
  let state_arc = Arc::clone(&STATE);
  let mut state = state_arc.lock().unwrap();
  let had_group = state.contains_key(group_name);
  if !had_group {
    state.insert(group_name, Arc::new(Mutex::new(BTreeMap::new())));
  }
  let g = state.get(group_name).unwrap();
  let log_group = Arc::clone(g);
  let mut f = log_group.lock().unwrap();
  f.insert(key, value);
}

pub fn set(key: &'static str, value: String) {
  set_group(ROOT_GROUP, key, value)
}