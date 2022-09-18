mod screen;
mod mouse;
mod Vector;
mod inventory_manager;
mod util;
mod gui;
mod app;
use termion::input::MouseTerminal;
use std::io::stdout;
use std::{thread};
use app::App;

fn main() {
    use termion::event::Key;
    use termion::input::TermRead;
    use termion::raw::IntoRawMode;
    use gui::{start};
    use std::io::{stdin};
    start();
    let stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    let stdin = stdin();
    thread::spawn(|| {
        let mut app = App::new();
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
