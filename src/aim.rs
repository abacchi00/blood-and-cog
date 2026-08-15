use macroquad::prelude::*;

const AIM_COLOR: Color = WHITE;
const AIM_BASE_LENGTH: f32 = 10.0;
const AIM_BASE_THICKNESS: f32 = 3.0;
const AIM_BASE_MOUSE_DISTANCE: f32 = 8.0;
const AIM_SCALE_AFTER_CLICK: f32 = 1.6;

pub struct Aim {
  scale: f32,
}

impl Aim {
  pub fn new() -> Self {
    Self { scale: 1.0 }
  }

  pub fn trigger_click(&mut self) {
    self.scale = AIM_SCALE_AFTER_CLICK;
  }

  pub fn update(&mut self, dt: f32) {
    // Smoothly animate the scale back to normal (1.0)
    self.scale = self.scale.lerp(1.0, dt * 10.0);
  }

  pub fn render(&self, mouse_x: f32, mouse_y: f32) {
    let dist_from_mouse = AIM_BASE_MOUSE_DISTANCE * self.scale;
    let length = AIM_BASE_LENGTH * self.scale;
    let thickness = AIM_BASE_THICKNESS * self.scale;
    let half_t = thickness / 2.0;

    // Left
    draw_rectangle(mouse_x - dist_from_mouse - length, mouse_y - half_t, length, thickness, AIM_COLOR);
    // Right
    draw_rectangle(mouse_x + dist_from_mouse, mouse_y - half_t, length, thickness, AIM_COLOR);
    // Top
    draw_rectangle(mouse_x - half_t, mouse_y - dist_from_mouse - length, thickness, length, AIM_COLOR);
    // Bottom
    draw_rectangle(mouse_x - half_t, mouse_y + dist_from_mouse, thickness, length, AIM_COLOR);
  }
}