use macroquad::prelude::*;

use crate::traits::{Renderable};
use crate::config::*;

pub struct Floor {
  pub pos: Vec2,
  width: f32,
  height: f32,
  has_dirt: bool,
}

impl Floor {
  pub fn new(opt_w: Option<f32>, opt_h: Option<f32>, opt_x: Option<f32>, opt_y: Option<f32>) -> Self {
    let width = opt_w.unwrap_or(20.0);
    let height = opt_h.unwrap_or(20.0);
    let x = opt_x.unwrap_or(rand::gen_range(width, screen_width() - width));
    let y = opt_y.unwrap_or(rand::gen_range(height, screen_height() - height));

    Self {  
      pos: Vec2::new(x, y),
      width,
      height,
      has_dirt: rand::gen_range(0.0, 100.0) < FLOOR_DIRT_CHANCE,
    }
  }
}

impl Renderable for Floor {
  fn render(&self) {
    draw_rectangle(self.pos.x, self.pos.y, self.width, self.height, FLOOR_COLOR);

    if self.has_dirt {
      draw_circle(self.pos.x + self.width/2.0, self.pos.y + self.height/2.0, self.width/4.0, FLOOR_DIRT_COLOR);
      draw_circle(self.pos.x + self.width/4.0, self.pos.y + self.height/4.0, self.width/6.0, FLOOR_DIRT_COLOR);
      draw_circle(self.pos.x + self.width/4.0, self.pos.y + self.height/1.5, self.width/8.0, FLOOR_DIRT_COLOR);
      draw_circle(self.pos.x + self.width/1.5, self.pos.y + self.height/1.6, self.width/7.0, FLOOR_DIRT_COLOR);
    }
  }
}
