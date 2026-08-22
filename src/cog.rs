use macroquad::prelude::*;

use crate::config::*;
use crate::traits::*;
use crate::alt_shapes::BorderedCircle;

pub struct Cog {
  pos: Vec2,
  collected: bool,
}

impl Cog {
  pub fn new(pos: Vec2) -> Self {
    Self {
      pos,
      collected: false
    }
  }

  pub fn mark_as_collected(&mut self) {
    self.collected = true;
  }
}

impl Renderable for Cog {
  fn render(&self) {
    // Cog shadow
    draw_circle(self.pos.x, self.pos.y + COG_SHADOW_OFFSET, COG_RADIUS, COG_SHADOW_COLOR);

    // Cog
    BorderedCircle {
      x: self.pos.x,
      y: self.pos.y,
      radius: COG_RADIUS,
      color: COG_SHADOW_COLOR,
      b_thick: COG_BORDER_THICKNESS,
      b_color: COG_COLOR,
    }.draw();
  }
}

impl Expirable for Cog {
  fn should_clean(&self) -> bool {
    self.collected
  }
}

impl Collidable for Cog {
  fn pos(&self) -> Vec2 { self.pos }
  fn shape(&self) -> CollisionShape {
    CollisionShape::Circle { radius: COG_RADIUS }
  }
}
