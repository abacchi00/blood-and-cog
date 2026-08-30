pub trait Damageable {
  fn health_mut(&mut self) -> &mut crate::health::Health;
  fn health(&self) -> &crate::health::Health;

  fn take_hit(&mut self) {
    self.health_mut().take_damage(20.0);
  }

  fn is_alive(&self) -> bool {
    self.health().is_alive()
  }
}

#[macro_export]
macro_rules! impl_damageable_enemy {
  ($struct_name:ident, $injured_color:expr) => {
    impl crate::traits::Damageable for $struct_name {
      fn health_mut(&mut self) -> &mut crate::health::Health { &mut self.hp }
      fn health(&self) -> &crate::health::Health { &self.hp }
    }

    impl crate::traits::Expirable for $struct_name {
      fn should_clean(&self) -> bool {
        !crate::traits::Damageable::health(self).is_alive()
      }
    }

    impl $struct_name {
      pub fn get_color(&self, color: macroquad::color::Color) -> macroquad::color::Color {
        if crate::traits::Damageable::health(self).is_injured() {
          return $injured_color;
        }
        color
      }
    }
  };
}

#[macro_export]
macro_rules! impl_damageable {
  ($struct_name:ident) => {
    impl crate::traits::Damageable for $struct_name {
      fn health_mut(&mut self) -> &mut crate::health::Health { &mut self.hp }
      fn health(&self) -> &crate::health::Health { &self.hp }
    }
  };
}
