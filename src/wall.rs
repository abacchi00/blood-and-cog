use macroquad::prelude::*;

use crate::config::*;
use crate::traits::{
  Renderable,
  Collidable, CollisionShape,
};
use crate::alt_shapes::BorderedRect;

pub struct Wall {
  pub pos: Vec2,
  width: f32,
  height: f32,
  variant: f32,
}

impl Wall {
  pub fn new(x: f32, y: f32) -> Self {
    Self {  
      pos: Vec2::new(x, y),
      width: GRID_CELL_SIZE,
      height: GRID_CELL_SIZE,
      variant: rand::gen_range(0.0, 100.0),
    }
  }
}

impl Renderable for Wall {
  fn render(&self) {
    let brick_w: f32 = self.width / 3.0;
    let brick_h: f32 = self.height / 6.0;

    // draw wall shadow
    draw_rectangle(
      self.pos.x, 
      self.pos.y + WALL_SHADOW_OFFSET_Y, 
      GRID_CELL_SIZE, 
      GRID_CELL_SIZE, 
      WALL_SHADOW_COLOR
    );
    
    draw_rectangle(self.pos.x, self.pos.y, self.width, self.height, WALL_COLOR);

    if self.variant < 20.0 {
      BorderedRect {
        x: self.pos.x + brick_w * 0.5,
        y: self.pos.y + brick_h,
        w: brick_w,
        h: brick_h,
        color: WALL_COLOR,
        b_thick: 1.0,
        b_color: WALL_BRICKS_COLOR,
      }.draw();
      BorderedRect {
        x: self.pos.x + brick_w,
        y: self.pos.y + brick_h * 2.0,
        w: brick_w,
        h: brick_h,
        color: WALL_COLOR,
        b_thick: 1.0,
        b_color: WALL_BRICKS_COLOR,
      }.draw();
      BorderedRect {
        x: self.pos.x + brick_w * 0.5,
        y: self.pos.y + brick_h * 3.0,
        w: brick_w,
        h: brick_h,
        color: WALL_COLOR,
        b_thick: 1.0,
        b_color: WALL_BRICKS_COLOR,
      }.draw();
    } else if self.variant < 40.0 {
      BorderedRect {
        x: self.pos.x + brick_w * 0.75,
        y: self.pos.y + brick_h * 3.0,
        w: brick_w,
        h: brick_h,
        color: WALL_COLOR,
        b_thick: 1.0,
        b_color: WALL_BRICKS_COLOR,
      }.draw();
      BorderedRect {
        x: self.pos.x + brick_w * 1.25,
        y: self.pos.y + brick_h * 4.0,
        w: brick_w,
        h: brick_h,
        color: WALL_COLOR,
        b_thick: 1.0,
        b_color: WALL_BRICKS_COLOR,
      }.draw();
    }
  }
}

impl Collidable for Wall {
  fn pos(&self) -> Vec2 { self.pos }
  fn shape(&self) -> CollisionShape {
    CollisionShape::Rectangle { w: self.width, h: self.height }
  }
}
