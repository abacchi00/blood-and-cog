use macroquad::prelude::*;

use crate::traits::{Collidable, CollisionShape, Expirable, Renderable};
use crate::config::*;

struct BloodBlob {
  offset: Vec2,
  radius: f32,
}

pub struct BloodPool {
  pos: Vec2,
  radius: f32,
  initial_radius: f32,
  lifetime: f32,
  blobs: Vec<BloodBlob>,
}

impl BloodPool {
  pub fn new(pos: Vec2) -> Self {
    let mut blobs = Vec::new();
    
    let seed_factor = (pos.x * 12.9898 + pos.y * 78.233).sin() * 43758.5453;
    let count = 4 + ((seed_factor.abs() % 3.0) as usize); // 4 to 6

    for i in 0..count {
      let angle = (i as f32) * (std::f32::consts::PI * 2.0 / count as f32) + (seed_factor * (i as f32 + 1.0)).cos();
      let dist = (seed_factor * (i as f32 + 2.0)).sin().abs() * 15.0; // Dist from center
      
      let offset = vec2(angle.cos() * dist, angle.sin() * dist);
      let blob_radius = 4.0 + ((seed_factor * (i as f32 + 3.0)).abs() % 4.0);

      blobs.push(BloodBlob { offset, radius: blob_radius });
    }

    Self {
      pos,
      radius: BLOOD_POOL_RADIUS,
      initial_radius: BLOOD_POOL_RADIUS,
      lifetime: BLOOD_POOL_LIFETIME,
      blobs,
    }
  }

  // Updates pool lifetime and dynamically scales down its collision/render radius
  pub fn update(&mut self, dt: f32) {
    if self.lifetime > 0.0 {
      self.lifetime -= dt;
      self.lifetime = self.lifetime.max(0.0);
      
      // Shrinks the pool size proportionally as it ages
      let life_ratio = (self.lifetime / BLOOD_POOL_LIFETIME).clamp(0.0, 1.0);
      self.radius = self.initial_radius * life_ratio;
    }
  }
}

impl Collidable for BloodPool {
  fn pos(&self) -> Vec2 {
    self.pos
  }
  fn shape(&self) -> CollisionShape {
    CollisionShape::Circle { radius: self.radius }
  }
}

impl Expirable for BloodPool {
  fn should_clean(&self) -> bool {
    self.lifetime <= 0.0 || self.radius <= 0.5
  }
}

impl Renderable for BloodPool {
  fn render(&self) {
    // Alpha fade based on remaining lifetime ratio
    let alpha = (self.lifetime / BLOOD_POOL_LIFETIME).clamp(0.0, 1.0);
    
    let scale_factor = (self.radius / self.initial_radius).clamp(0.0, 1.0);

    for blob in &self.blobs {
      let current_blob_radius = blob.radius * scale_factor;
      let blob_pos = self.pos + (blob.offset * scale_factor);

      draw_circle(
        blob_pos.x,
        blob_pos.y,
        current_blob_radius,
        BLOOD_POOL_COLOR.with_alpha(alpha),
      );
    }
  }
}