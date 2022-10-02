
use std::io::{Write, stdout};
use std::sync::mpsc::{Sender, channel, Receiver};
use std::thread;
use std::time::Duration;
use std::sync::{Arc};
use std::sync::Mutex;
use termion::{color, style};
use std::collections::BTreeMap;

static ROOT_GROUP: &str = "$$static$$";
pub struct Logs {}
impl Logs {
  pub const RunCountTotal: &'static str = "Run count (total)";
  pub const RunCountCurrent: &'static str = "Run count (current)";
  pub const AppStatus: &'static str = "App status";
  pub const MousePosition: &'static str = "Mouse pos";
  pub const PointInBounds: &'static str = "Mouse in position";
  pub const MagicMenuFocused: &'static str = "Magic menu open";
}

struct GuiMessage (&'static str, &'static str, String);
fn GuiRenderer() -> Sender<GuiMessage> {
  let (sender, receiver): (Sender<GuiMessage>, Receiver<GuiMessage>) = channel();
  let state_global: Arc<Mutex<BTreeMap<&'static str, Arc<Mutex<BTreeMap<&'static str, String>>>>>>;
  let mut root_btree = BTreeMap::new();
  root_btree.insert(ROOT_GROUP, Arc::new(Mutex::new(BTreeMap::new())));
  state_global = Arc::new(Mutex::new(root_btree));

  let sa = Arc::clone(&state_global);
  let sb = Arc::clone(&state_global);
  
  thread::spawn(move || {
    for event in receiver.iter() {
      let group_name = event.0;
      let key = event.1;
      let value = event.2;
      let mut state = sa.lock().unwrap();
      let had_group = state.contains_key(group_name);
      if !had_group {
        state.insert(group_name, Arc::new(Mutex::new(BTreeMap::new())));
      }
      let g = state.get(group_name).unwrap();
      let log_group = Arc::clone(g);
      let mut f = log_group.lock().unwrap();
      f.insert(key, value);
      drop(state);
    }
  });
  thread::spawn(move || {
    loop {
      thread::sleep(Duration::from_millis(30));
      use termion::color::*;
      use termion::clear;
      use termion::cursor::{Goto, Show};
      let state = sb.lock().unwrap();
      // let state = state_global.lock().unwrap();
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
      drop(state);
    }
  });
  sender
}
pub struct Gui {
  sender: Sender<GuiMessage>,
}

impl Clone for Gui {
  fn clone(&self) -> Gui {
    Gui { sender: self.sender.clone() }
  }
}

impl Gui {
  pub fn new() -> Self {
    Gui {
      sender: GuiRenderer(),
    }
  }
  pub fn set_group(&self, group_name: &'static str, key: &'static str, value: String) {
    self.sender.send(GuiMessage(group_name, key, value)).unwrap();
  }
  pub fn set(&self, key: &'static str, value: String) {
    self.set_group(ROOT_GROUP, key, value);
  }
}