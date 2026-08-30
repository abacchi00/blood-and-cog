use macroquad::prelude::*;

use crate::config::*;
use crate::traits::{Collidable, CollisionShape, Expirable, Renderable};

pub struct Leechy {
  pub pos: Vec2,
  pub radius: f32,
  life: f32,
  facing_angle: f32,
  pulse_timer: f32,
  injured_cooldown: f32,
  trail_timer: f32,
}

impl Leechy {
  pub fn new(pos: Vec2) -> Self {
    let seed = (pos.x * 12.9898 + pos.y * 78.233).sin() * 43758.5453;
    let size_multiplier = 0.6 + ((seed.abs() % 0.4));

    Self {
      pos,
      radius: LEECHY_RADIUS * size_multiplier,
      life: LEECHY_INITIAL_LIFE * size_multiplier,
      facing_angle: 0.0,
      pulse_timer: 0.0,
      injured_cooldown: 0.0,
      trail_timer: 0.0,
    }
  }

  pub fn update(&mut self, player_pos: Vec2, dt: f32) -> (Vec2, Option<Vec2>) {
    if self.injured_cooldown > 0.0 {
      self.injured_cooldown -= dt;
      self.injured_cooldown = self.injured_cooldown.max(0.0);
    }

    let direction = player_pos - self.pos;
    let mut spawned_blood = None;

    if direction.length_squared() > 0.001 {
      self.facing_angle = direction.y.atan2(direction.x);
      self.pulse_timer += dt * 5.0;

      self.trail_timer += dt;
      if self.trail_timer >= LEECHY_TRAIL_INTERVAL {
        self.trail_timer = 0.0;
        spawned_blood = Some(self.pos);
      }
    }

    let normalized_dir = direction.normalize_or_zero();
    let delta = normalized_dir * LEECHY_BASE_SPEED * dt;

    (delta, spawned_blood)
  }

  pub fn take_hit(&mut self) {
    if self.injured_cooldown <= 0.0 {
      self.injured_cooldown = LEECHY_INJURED_DURATION;
    }
    self.life -= 20.0;
    self.life = self.life.max(0.0);
  }

  pub fn is_alive(&self) -> bool {
    self.life > 0.0
  }

  fn get_color(&self, color: Color) -> Color {
    if self.injured_cooldown > 0.0 { return LEECHY_INJURED_COLOR; }

    color
  }
}

impl Collidable for Leechy {
  fn pos(&self) -> Vec2 {
    self.pos
  }
  fn shape(&self) -> CollisionShape {
    CollisionShape::Circle { radius: self.radius * 0.9 }
  }
}

impl Expirable for Leechy {
  fn should_clean(&self) -> bool {
    !self.is_alive()
  }
}

impl Renderable for Leechy {
  fn render(&self) {
    let radius = self.radius;
    let dir_vec = vec2(self.facing_angle.cos(), self.facing_angle.sin());
    let perp_vec = vec2(-dir_vec.y, dir_vec.x);
    let stretch = self.pulse_timer.sin() * 0.40;

    let segment_count = 5;

    // Global shadow
    let shadow_offset = vec2(2.0, 3.0);
    for i in 0..segment_count {
      let t = i as f32 / (segment_count - 1) as f32;
      let offset_dist = (t - 0.5) * (radius * 2.4 * (1.0 + stretch));
      let seg_x = self.pos.x + dir_vec.x * offset_dist + shadow_offset.x;
      let seg_y = self.pos.y + dir_vec.y * offset_dist + shadow_offset.y;

      let width_profile = 1.0 - (t - 0.5).abs() * 1.4;
      let seg_radius = radius * (0.5 + width_profile * 0.6);

      draw_circle(
        seg_x, seg_y,
        seg_radius,
        LEECHY_SHADOW_COLOR,
      );
    }

    // Segmented body
    let mut head_pos = self.pos;

    for i in 0..segment_count {
      let t = i as f32 / (segment_count - 1) as f32;
      let offset_dist = (t - 0.5) * (radius * 2.4 * (1.0 + stretch));
      let seg_x = self.pos.x + dir_vec.x * offset_dist;
      let seg_y = self.pos.y + dir_vec.y * offset_dist;

      if i == segment_count - 1 {
        head_pos = vec2(seg_x, seg_y);
      }

      let width_profile = 1.0 - (t - 0.5).abs() * 1.4;
      let seg_radius = radius * (0.5 + width_profile * 0.6);

      draw_circle(
        seg_x, seg_y,
        seg_radius,
        self.get_color(LEECHY_BODY_COLOR),
      );
    }

    // Eyes and tentacles integrated to head
    for i in -1..=1 {
      let side_offset = (i as f32) * 3.0;
      let base_tentacle = head_pos + perp_vec * side_offset + dir_vec * 2.0;
      
      let tentacle_angle = self.facing_angle + (i as f32) * 0.3;
      let tentacle_length = 5.0 + (self.pulse_timer * 1.5 + i as f32).cos().abs() * 3.5;

      let tip_x = base_tentacle.x + tentacle_angle.cos() * tentacle_length;
      let tip_y = base_tentacle.y + tentacle_angle.sin() * tentacle_length;

      draw_line(
        base_tentacle.x,
        base_tentacle.y,
        tip_x,
        tip_y,
        1.8,
        self.get_color(LEECHY_TENTACLE_COLOR),
      );
    }

    // Eyes in head with gloss
    for i in [-1.0, 1.0] {
      let eye_pos = head_pos + perp_vec * (i * 6.0) + dir_vec * 1.0;

      draw_circle(eye_pos.x, eye_pos.y, 5.0, self.get_color(LEECHY_BODY_COLOR));
      draw_circle(eye_pos.x, eye_pos.y, 4.0, self.get_color(LEECHY_EYE_BASE_COLOR));
      
      let gloss_pos = eye_pos - perp_vec * 0.8 - dir_vec * 1.2;
      draw_circle(gloss_pos.x, gloss_pos.y, 1.2, self.get_color(LEECHY_EYE_GLOSS_COLOR));
    }
  }
}
