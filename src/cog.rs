use macroquad::prelude::*;

use crate::traits::*;
use crate::alt_shapes::BorderedCircle;

pub struct Cog {
  pub pos: Vec2,
  pub collided: bool,
}

impl Cog {
  pub fn new(pos: Vec2) -> Self {
    Self { pos, collided: false }
  }
}

impl Renderable for Cog {
  fn render(&self) {
    let circ_radius = 5.0f32;

    // Cog shadow
    draw_circle(self.pos.x, self.pos.y + 3.0, circ_radius, BLACK);

    // Cog
    BorderedCircle {
      x: self.pos.x,
      y: self.pos.y,
      radius: circ_radius,
      color: BLACK,
      b_thick: 3.0,
      b_color: LIGHTGRAY,
    }.draw();
  }
}

impl Expirable for Cog {
  fn should_clean(&self) -> bool {
    self.collided
  }
}

impl Collidable for Cog {
  fn pos(&self) -> Vec2 { self.pos }
  fn shape(&self) -> CollisionShape {
    CollisionShape::Circle { radius: 5.0 }
  }
}
