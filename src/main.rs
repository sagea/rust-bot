mod screen;
mod mouse;
mod inventory_manager;
mod util;
mod gui;
mod app;
mod vector;

use termion::input::MouseTerminal;
use std::io::stdout;
use std::{thread};
use app::App;

fn main() {
    use termion::event::Key;
    use termion::input::TermRead;
    use termion::raw::IntoRawMode;
    use std::io::{stdin};
    let stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    let stdin = stdin();
    let gui = gui::Gui::new();
    thread::spawn(|| {
        let mut app = App::new(gui);
        app.start();
    });
    for c in stdin.keys() {
        let val = c.unwrap();
        match val {
            Key::Char('q') => break,
            Key::Ctrl('c') => break,
            _ => {}
        }
    }
    drop(stdout);
}
