use std::cell::RefCell;
use std::io::{Write, Stdout, stdout};


// use crossbeam::channel::{unbounded, Sender};
use lazy_static::lazy_static;

use termion::raw::{RawTerminal, IntoRawMode};
use termion::input::{MouseTerminal};
// use std::rc::{Weak, Rc};
use std::sync::{Arc};
use std::sync::Mutex;
use termion::{color, style};


use std::collections::BTreeMap;


static ROOT_GROUP: &str = "$$static$$";

lazy_static! {
  static ref STATE: Arc<Mutex<BTreeMap<&'static str, Arc<Mutex<BTreeMap<&'static str, String>>>>>> = {
    // let ref_owner: Rc<BTreeMap<&'static str, String>> = 
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



#[derive(Debug, PartialEq)]
enum RenderState {
  Full,
  SingleLine,
  None,
}

pub fn set_group(group_name: &'static str, key: &'static str, value: String) {
  let mut group_map = STATE.lock().unwrap();
  let had_group = group_map.contains_key(group_name);
  if !had_group {
    group_map.insert(group_name, Arc::new(Mutex::new(BTreeMap::new())));
  }
  let g = group_map.get(group_name).unwrap();
  let log_group = Arc::clone(g);
  let mut f = log_group.lock().unwrap();
  let res = f.insert(key, value.clone());
  let render_state = match res {
      None => RenderState::Full,
      Some(old_value) => {
        if old_value == value {
          RenderState::None
        } else {
          RenderState::SingleLine
        }
      }
  };
  drop(f);
  if render_state == RenderState::None {
    return ;
  }

  use termion::color::*;
  use termion::clear;
  use termion::cursor::{Goto, Show};
  let mut stdout = stdout();

  if render_state == RenderState::Full {
    write!(stdout, "{}", clear::All).unwrap();
  }
  let mut offsety = 5;
  let mut test_offset = 5;
  let mut change = 1;
  for (name, group) in group_map.iter() {
    let cloned = Arc::clone(group);
    let gg = cloned.lock().unwrap();
    let is_root_group = name.to_string().as_str() == "$$static$$";
    let group_names_match = *name != group_name;
    let _header_offset_y: u16 = if is_root_group { 0 } else { 1 };
    let offsetx: u16 = if is_root_group { 0 } else { 3 };
    if !is_root_group {
      offsety += 1;
      write!(stdout, "{}{}", Goto(0, offsety), Show).unwrap();
      write!(stdout, "{}{}{}", Fg(Red), name, style::Reset).unwrap();
      write!(stdout, "{}", clear::UntilNewline).unwrap();
      write!(stdout, "{}", Goto(0, offsety + 1)).unwrap();
    }

    test_offset += if !group_names_match {
      change += 10;
      let diff = gg.len() as u16;
      write!(stdout, "{} val {} {}", Goto(change, 1), test_offset, diff).unwrap();
      diff
    } else { 0 };
    write!(stdout, "{} val {}", Goto(change, 2), test_offset).unwrap();
    for (prop, value) in gg.iter().rev() {
      offsety += 1;
      write!(stdout, "{}{}", Goto(offsetx, offsety), Show).unwrap();
      if render_state == RenderState::SingleLine {
        if group_names_match {
          continue;
        }
        if *prop != key {
          continue;
        }
      }
      write!(stdout, "{}", clear::UntilNewline).unwrap();
      write!(stdout, "{}{}{}: {}", color::Fg(color::Red), prop, style::Reset, value).unwrap();
      // write!(stdout, "{}", clear::UntilNewline).unwrap();
      write!(stdout, "{}{} expected:{} actual: {}", Goto(1, 3), clear::UntilNewline, offsety, test_offset).unwrap();
    }
  }
  write!(stdout, "{}", Goto(1, offsety + 1)).unwrap();
  write!(stdout, "{}", clear::AfterCursor).unwrap();
  stdout.flush().unwrap();
}

pub fn set(key: &'static str, value: String) {
  set_group(ROOT_GROUP, key, value)
}

pub fn init(pos: MouseTerminal<RawTerminal<Stdout>>) {
  // let outp = output.lock().unwrap();
  // outp.replace(Some(pos));
}

pub fn reset() {
  // use ;
  // let mut stdout = output.lock().unwrap();
  // stdout.suspend_raw_mode().unwrap();
  // stdout.flush().unwrap();
  // core::mem::drop(stdout);
}