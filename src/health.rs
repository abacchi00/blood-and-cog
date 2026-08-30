pub struct Health {
  pub current: f32,
  // pub max: f32,
  pub injured_cooldown: f32,
  injured_duration: f32,
}

impl Health {
  pub fn new(max: f32, injured_duration: f32) -> Self {
    Self {
      current: max,
      // max,
      injured_cooldown: 0.0,
      injured_duration,
    }
  }

  pub fn update(&mut self, dt: f32) {
    if self.injured_cooldown > 0.0 {
      self.injured_cooldown -= dt;
      self.injured_cooldown = self.injured_cooldown.max(0.0);
    }
  }

  pub fn take_damage(&mut self, amount: f32) {
    if self.injured_cooldown <= 0.0 {
      self.current = (self.current - amount).max(0.0);
      self.injured_cooldown = self.injured_duration;
    }
  }

  pub fn is_alive(&self) -> bool {
    self.current > 0.0
  }

  pub fn is_injured(&self) -> bool {
    self.injured_cooldown > 0.0
  }
}
