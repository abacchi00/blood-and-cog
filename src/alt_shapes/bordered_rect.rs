use macroquad::prelude::*;

pub struct BorderedRect {
  pub x: f32,
  pub y: f32,
  pub w: f32,
  pub h: f32,
  pub color: Color,
  pub b_thick: f32,
  pub b_color: Color,
}

impl BorderedRect {
  pub fn draw(&self) {
    draw_rectangle(self.x, self.y, self.w, self.h, self.b_color);

    let inner_x = self.x + self.b_thick;
    let inner_y = self.y + self.b_thick;
    let inner_w = (self.w - self.b_thick * 2.0).max(0.0);
    let inner_h = (self.h - self.b_thick * 2.0).max(0.0);

    draw_rectangle(inner_x, inner_y, inner_w, inner_h, self.color);
  }
}
