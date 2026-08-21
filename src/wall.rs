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
  pub fn new(x: f32, y: f32) -> Self {
    Self {  
      pos: Vec2::new(x, y),
      width: GRID_CELL_SIZE,
      height: GRID_CELL_SIZE,
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
