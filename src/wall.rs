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
  pub fn new() -> Self {
    let width = 20.0;
    let height = 20.0;
    
    Self {  
      pos: Vec2::new(
        rand::gen_range(width, screen_width() - width),
        rand::gen_range(height, screen_height() - height),
      ),
      width,
      height,
    }
  }
}

impl Renderable for Wall {
  fn render(&self) {
    draw_rectangle(self.pos.x - self.width/2.0, self.pos.y - self.height/2.0, self.width, self.height, WALL_COLOR);
  }
}

impl Collidable for Wall {
  fn pos(&self) -> Vec2 { self.pos }
  fn shape(&self) -> CollisionShape {
    CollisionShape::Rectangle { w: self.width, h: self.height }
  }
}
