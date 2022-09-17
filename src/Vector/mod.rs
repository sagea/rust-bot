use std::cmp::{max, min};

pub struct Rect {
  pub x1: i32,
  pub y1: i32,
  pub x2: i32,
  pub y2: i32,
  pub width: i32,
  pub height: i32,
}

impl Rect {
  pub fn from_size(x: i32, y: i32, width: i32, height: i32) -> Rect {
    Rect {
      x1: x,
      y1: y,
      x2: x + width,
      y2: y + height,
      width,
      height,
    }
  }
  pub fn from_size_tupl((x1, y1): (i32, i32), (width, height): (i32, i32)) -> Rect {
    Rect::from_size(x1, y1, width, height)
  }
  pub fn from_points(x1: i32, y1: i32, x2: i32, y2: i32) -> Rect {
    Rect {
      x1, y1, x2, y2,
      width: x2 - x1,
      height: y1 - y2,
    }
  }
  pub fn from_points_tupl((x1, y1): (i32, i32), (x2, _y2): (i32, i32)) -> Rect {
    Rect::from_points(x1, y1, x2, y1)
  }
  pub fn overlap(a: Rect, b: Rect) -> Option<Rect> {
    if a.x1 >= b.x2 { return None; }
    if b.x1 >= a.x2 { return None; }
    if a.y1 >= b.y2 { return None; }
    if b.y1 >= a.y2 { return None; }
    let x1 = max(a.x1, b.x1);
    let y1 = max(a.y1, b.y1);
    let x2 = min(a.x2, b.x2);
    let y2 = min(a.y2, b.y2);
    Some(Rect::from_points(x1, y1, x2, y2))
  }
  pub fn point_inside(rect: &Rect, x: i32, y: i32) -> bool {
    if rect.x1 > x || x > rect.x2 { false }
    else { !(rect.y1 > y || y > rect.y2) }
  }

  pub fn point_inside_tupl(rect: &Rect, (x, y): (i32, i32)) -> bool {
    Rect::point_inside(rect, x, y)
  }
}
