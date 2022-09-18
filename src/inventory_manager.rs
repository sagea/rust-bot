
use crossbeam::channel::{unbounded, Receiver};

use crate::screen;
use crate::vector;

use crate::gui;

use std::thread;

#[derive(Debug, Clone, Copy)]

pub enum MenuItemTrackerState {
  Active,
  Inactive,
}
use std::ops::Sub;


pub fn get_luminosity(r: u8, g: u8, b: u8) -> f32 {
  let _r = r as f32;
  let _g = g as f32;
  let _b = b as f32;
  let val = 0.299_f32 * _r.powf(2.0) + 0.587 * _g.powf(2.0) + 0.114_f32 * _b.powf(2.0);
  val.sqrt()
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ImagePixelResults {
  pub luminosity: f32,
  pub r_avg: f32,
  pub g_avg: f32,
  pub b_avg: f32,
  pub avg_total: f32,

  pub r_perc: f32,
  pub g_perc: f32,
  pub b_perc: f32,
  pub perc_total: f32,
}
impl Sub for ImagePixelResults {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self::Output {
    ImagePixelResults {
      luminosity: self.luminosity - rhs.luminosity,
      r_avg: self.r_avg - rhs.r_avg,
      g_avg: self.g_avg - rhs.g_avg,
      b_avg: self.b_avg - rhs.b_avg,
      avg_total: self.avg_total - rhs.avg_total,
      r_perc: self.r_perc - rhs.r_perc,
      g_perc: self.g_perc - rhs.g_perc,
      b_perc: self.b_perc - rhs.b_perc,
      perc_total: self.perc_total - rhs.perc_total,
    }
  }
}
impl ImagePixelResults {
    pub fn from_pixel_bytes(pixel_bytes: &Vec<u8>) -> ImagePixelResults {
      let mut b_total: i32 = 0;
      let mut g_total: i32 = 0;
      let mut r_total: i32 = 0;
      let px_count = (pixel_bytes.len() / 4) as i32; 
      let mut luminosity_total = 0 as f32;
      for i in (0..pixel_bytes.len()).step_by(4) {
        let r = pixel_bytes[i + 2];
        let g = pixel_bytes[i + 1];
        let b = pixel_bytes[i];
        r_total += r as i32;
        g_total += g as i32;
        b_total += b as i32;
        luminosity_total = get_luminosity(r, g, b)
      }
      let r_avg = r_total as f32 / px_count as f32;
      let g_avg = g_total as f32 / px_count as f32;
      let b_avg = b_total as f32 / px_count as f32;
      let avg_total = (r_avg + b_avg + g_avg) as f32;
      let luminosity = luminosity_total / px_count as f32;
      let r_perc = r_avg / avg_total;
      let g_perc = g_avg / avg_total;
      let b_perc = b_avg / avg_total;
      ImagePixelResults {
        luminosity,
        r_avg,
        g_avg,
        b_avg,
        avg_total,
        r_perc,
        g_perc,
        b_perc,
        perc_total: r_perc + g_perc + b_perc,
      }
    }
    pub fn zero() -> ImagePixelResults {
      ImagePixelResults::from_pixel_bytes(&vec![0, 0, 0, 0])
    }
    pub fn to_avgs_string(&self) -> String {
      format!(
        "r {: >7}     g {: >7}    b {: >7}    l {: >7}",
        format!("{:.2}", self.r_avg), 
        format!("{:.2}", self.g_avg), 
        format!("{:.2}", self.b_avg),
        format!("{:.2}", self.luminosity),
      )
    }
    pub fn to_perc_string(&self) -> String {
      format!("r {:.2} - g {:.2} - b {:.2}", self.r_perc, self.g_perc, self.b_perc)
    }
}

fn is_slot_active(pixels: &ImagePixelResults) -> bool {
  pixels.r_perc > 0.5
}

pub fn magic_menu_status_tracker() -> Receiver<bool> {
  use vector::Rect;
  let _top = 237;
  // 270
  // 30
  // let spell = Rect::from_size(1624, top, 5, 5);
  // 1419 1453 1486 1521 1555 1588 1621
  //    34   33   36    34
  // 12 51
  // menu_active_tracker("Attack", Rect::from_size(1419, top, 5, 5));
  // menu_active_tracker("Stats", Rect::from_size(1453, top, 5, 5));
  // menu_active_tracker("QuestList", Rect::from_size(1486, top, 5, 5));
  // menu_active_tracker("Inventory", Rect::from_size(1521, top, 5, 5));
  // menu_active_tracker("Equipment", Rect::from_size(1555, top, 5, 5));
  // menu_active_tracker("Prayer", Rect::from_size(1588, top, 5, 5));
  menu_active_tracker("Magic", Rect::from_size(1658, 238, 5, 5))
}

pub fn menu_active_tracker(logGroup: &'static str, rect: vector::Rect) -> Receiver<bool> {
  use crate::util::sleep_exact;
  
  
  let (sender, receiver) = unbounded();
  thread::spawn(move || {
    let mut last_state = 0;
    let mut og = vec![
      ImagePixelResults::zero(),
      ImagePixelResults::zero(),
    ];
    let _draw = 0;
    loop {
      let screenshot = screen::screenshot(rect.x1, rect.y1, rect.width, rect.height);
      
      if let Ok(image) = screenshot {
        let cur_px_details = ImagePixelResults::from_pixel_bytes(&image.pixels);
        if og[1] == cur_px_details {
          // gui::set_group("Pixels ggggg", "bro ", "equals".to_string());
        } else {
          // gui::set_group("Pixels ggggg", "bro ", "notequals".to_string());
          let f = &mut og;
          f[0] = f[1];
          f[1] = cur_px_details;
        }
        let a = og[0];
        let b = og[1];
        let diff = a - b;
        // let dif_px_details = og[0] - og[1];
        gui::set_group(logGroup.clone(), "last", og[0].to_avgs_string());
        gui::set_group(logGroup.clone(), "now ", cur_px_details.to_avgs_string());
        gui::set_group(logGroup.clone(), "diff", diff.clone().to_avgs_string());

        // gui::set_group("Pixels (avg)", "last2", og[1].to_avgs_string());

        if og[1] == cur_px_details {
          // gui::set_group("Pixels ggggg", "bro ", "equals".to_string());
        } else {
          // gui::set_group("Pixels ggggg", "bro ", "notequals".to_string());
          let f = &mut og;
          f[0] = f[1];
          f[1] = cur_px_details;
        }

        let mut cur_state = 0;
        if is_slot_active(&cur_px_details) {
          cur_state = 1;
        } else {
          cur_state = 2;
        }

        if last_state != cur_state {
          // println!("draw {} {}", last_state, cur_state);
          last_state = cur_state;

          if cur_state == 1 {
            sender.send(true).unwrap();
          } else if cur_state == 2 {
            sender.send(false).unwrap();
          }
        }
      } else {
        println!("Error imageError");
        // std::io::_print
      }
      sleep_exact(50)
    }
  });

  receiver
}