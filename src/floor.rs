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
  pub fn new(x: f32, y: f32) -> Self {
    Self {  
      pos: Vec2::new(x, y),
      width: GRID_CELL_SIZE,
      height: GRID_CELL_SIZE,
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
