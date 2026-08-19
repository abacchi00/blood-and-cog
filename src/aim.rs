use macroquad::prelude::*;

use crate::traits::{Renderable, Updatable};

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
}

impl Renderable for Aim {
  fn render(&self) {
    let (mx, my) = mouse_position();

    let dist_from_mouse = AIM_BASE_MOUSE_DISTANCE * self.scale;
    let length = AIM_BASE_LENGTH * self.scale;
    let thickness = AIM_BASE_THICKNESS * self.scale;
    let half_t = thickness / 2.0;

    // Left
    draw_rectangle(mx - dist_from_mouse - length, my - half_t, length, thickness, AIM_COLOR);
    // Right
    draw_rectangle(mx + dist_from_mouse, my - half_t, length, thickness, AIM_COLOR);
    // Top
    draw_rectangle(mx - half_t, my - dist_from_mouse - length, thickness, length, AIM_COLOR);
    // Bottom
    draw_rectangle(mx - half_t, my + dist_from_mouse, thickness, length, AIM_COLOR);
  }
}

impl Updatable for Aim {
  fn update(&mut self, dt: f32, _world_width: f32, _world_height: f32) {
    // Smoothly animate the scale back to normal (1.0)
    self.scale = self.scale.lerp(1.0, dt * 10.0);
  }
}
