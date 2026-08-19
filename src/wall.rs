use macroquad::prelude::*;

use crate::traits::{
  Renderable,
  Collidable, CollisionShape,
};
use crate::config::*;

pub struct Wall {
  pub pos: Vec2,
  width: f32,
  height: f32,
}

impl Wall {
  pub fn new(opt_w: Option<f32>, opt_h: Option<f32>, opt_x: Option<f32>, opt_y: Option<f32>) -> Self {
    let width = opt_w.unwrap_or(20.0);
    let height = opt_h.unwrap_or(20.0);
    let x = opt_x.unwrap_or(rand::gen_range(width, screen_width() - width));
    let y = opt_y.unwrap_or(rand::gen_range(height, screen_height() - height));

    Self {  
      pos: Vec2::new(x, y),
      width,
      height,
    }
  }
}

impl Renderable for Wall {
  fn render(&self) {
    draw_rectangle(self.pos.x, self.pos.y, self.width, self.height, WALL_COLOR);
  }
}

impl Collidable for Wall {
  fn pos(&self) -> Vec2 { self.pos }
  fn shape(&self) -> CollisionShape {
    CollisionShape::Rectangle { w: self.width, h: self.height }
  }
}
