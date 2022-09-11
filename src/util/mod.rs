pub fn sleep (from: i32, to: i32) {
  use rand::Rng;
  use std::{thread, time};
  let mut rng = rand::thread_rng();
  let random = rng.gen_range(from..to);
  thread::sleep(time::Duration::from_millis(random as u64));
}
pub fn sleep_exact (duration: i32) {
  use std::{thread, time};
  thread::sleep(time::Duration::from_millis(duration as u64));
}
