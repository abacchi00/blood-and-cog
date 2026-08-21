use macroquad::prelude::*;

use crate::traits::{Renderable};
use crate::config::*;

pub struct Floor {
  pub pos: Vec2,
  width: f32,
  height: f32,
  variant: f32,
  variant_color: f32,
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
      variant: rand::gen_range(0.0, 100.0),
      variant_color: rand::gen_range(0.0, 100.0),
    }
  }
}

impl Renderable for Floor {
  fn render(&self) {
    draw_rectangle(self.pos.x, self.pos.y, self.width, self.height, FLOOR_COLOR);

    let color = match self.variant_color {
      0.0..80.0 => { FLOOR_DIRT_COLOR },
      80.0..90.0 => { Color::from_hex(0x332e2a) },
      90.0..=100.0 => { Color::from_hex(0x2a3230) },
      _ => FLOOR_DIRT_COLOR,
    };


    match self.variant {
      0.0..5.0 => {
        draw_circle(self.pos.x + self.width/2.0, self.pos.y + self.height/2.0, self.width/4.0, color);
        draw_circle(self.pos.x + self.width/4.0, self.pos.y + self.height/4.0, self.width/6.0, color);
        draw_circle(self.pos.x + self.width/4.0, self.pos.y + self.height/1.5, self.width/8.0, color);
        draw_circle(self.pos.x + self.width/1.5, self.pos.y + self.height/1.6, self.width/7.0, color);
      },
      5.0..10.0 => {
        draw_circle(self.pos.x + self.width/1.6, self.pos.y + self.height/4.0, self.width/5.0, color);
        draw_circle(self.pos.x + self.width/2.0, self.pos.y + self.height/1.5, self.width/6.0, color);
        draw_circle(self.pos.x + self.width/3.0, self.pos.y + self.height/1.6, self.width/4.0, color);
      },
      10.0..15.0 => {
        draw_circle(self.pos.x + self.width/4.6, self.pos.y + self.height/3.0, self.width/5.0, color);
        draw_circle(self.pos.x + self.width/2.0, self.pos.y + self.height/2.0, self.width/6.0, color);
        draw_circle(self.pos.x + self.width/3.0, self.pos.y + self.height/1.2, self.width/6.0, color);
      }
       _ => ()
    }
  }
}
