pub use display_info::DisplayInfo;
use std::{ cmp };
use core_graphics::display::{CGDisplay, CGPoint, CGRect, CGSize };

pub struct Image {
  pub width: i32,
  pub height: i32,
  pub pixels: Vec<u8>,
}

#[derive(Debug)]
pub enum ImageErrors {
  invalid_image,
}
pub fn screenshot(x: i32, y: i32, width: i32, height: i32) -> Result<Image, ImageErrors> {
  let displays = DisplayInfo::all().unwrap();
  for di in displays.iter() {

    let screen_x2 = di.x + di.width as i32;
    let screen_y2 = di.y + di.height as i32;

    let mut x1 = x + di.x;
    let mut y1 = y + di.y;
    let mut x2 = x1 + width as i32;
    let mut y2 = y1 + height as i32;
    x1 = cmp::max(di.x, cmp::min(screen_x2, x1));
    y1 = cmp::max(di.y, cmp::min(screen_y2, y1));
    x2 = cmp::min(x2, screen_x2);
    y2 = cmp::min(y2, screen_y2);
    let diffx = x2 - x1;
    let diffy = y2 - y1;

    if x1 >= x2 || y1 >= y2 {
      return Err(ImageErrors::invalid_image);
    }
    let cg_display = CGDisplay::new(di.id);
    let full_cg_image = cg_display.image().unwrap();

    let w = width as f32;// * di.scale_factor;
    let h = height as f32;// * di.scale_factor;
    
    let cg_rect = CGRect::new(
      &CGPoint::new((x as f32 * di.scale_factor) as f64, (y as f32 * di.scale_factor) as f64),
      &CGSize::new(w as f64, h as f64),
    );
    let cg_image = full_cg_image.cropped(cg_rect).unwrap();
    let mut bgra_image = Vec::from(cg_image.data().bytes());
    return Ok(Image {
      width: cg_image.width() as i32,
      height: cg_image.height() as i32,
      pixels: bgra_image,
    })
  }
  return Err(ImageErrors::invalid_image);
}
