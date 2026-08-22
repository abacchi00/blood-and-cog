use macroquad::prelude::*;

pub struct BorderedCircle {
  pub x: f32,
  pub y: f32,
  pub radius: f32,
  pub color: Color,
  pub b_thick: f32,
  pub b_color: Color,
}

impl BorderedCircle {
  pub fn draw(&self) {
    draw_circle(self.x, self.y, self.radius, self.b_color);
    draw_circle(self.x, self.y, self.radius - self.b_thick, self.color);
  }
}
