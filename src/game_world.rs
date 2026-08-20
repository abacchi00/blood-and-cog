use macroquad::prelude::*;

use crate::traits::{
  Renderable,
  Updatable,
  Expirable,
  Collidable, 
  check_collision,
};
use crate::config::*;

use crate::player::Player;
use crate::bullet::Bullet;
use crate::aim::Aim;
use crate::enemy::Enemy;
use crate::arena::Arena;
use crate::game_command::GameCommand;

pub struct GameWorld {
  player: Player,
  aim: Aim,
  bullets: Vec<Bullet>,
  enemies: Vec<Enemy>,
  arena: Arena,
}

impl GameWorld {
  pub fn new() -> Self {
    let arena = Arena::new();
    let initial_player_pos = arena.initial_player_pos;

    let mut world = Self {
      player: Player::new(initial_player_pos),
      aim: Aim::new(),
      bullets: Vec::new(),
      enemies: Vec::new(),
      arena,
    };

    world.spawn_initial_enemies();
    world
  }

  fn spawn_initial_enemies(&mut self) {
    for _ in 0..4 {
      self.enemies.push(Enemy::new(self.arena.random_available_position()));
    }
  }

  pub fn handle_input(&mut self) -> GameCommand {
    if is_key_down(KeyCode::Escape) { return GameCommand::Exit; };

    self.player.update_input();

    let screen_mouse_pos = Vec2::from(mouse_position());
    let half_screen = vec2(screen_width() / 2.0, screen_height() / 2.0);
    let offset_from_center = screen_mouse_pos - half_screen;
    let world_mouse_pos = self.player.pos + offset_from_center;

    if is_mouse_button_pressed(MouseButton::Left) {
      self.bullets.push(Bullet::new(self.player.pos, world_mouse_pos));
      self.aim.trigger_click();
    }

    GameCommand::Continue
  }

  pub fn update(&mut self, dt: f32) {
    let sw = screen_width();
    let sh = screen_height();

    self.player.update(dt, sw, sh);
    self.aim.update(dt, sw, sh);
    
    for bullet in &mut self.bullets { bullet.update(dt, sw, sh); }
    for enemy in &mut self.enemies { enemy.update(dt, sw, sh); }

    self.resolve_collisions();
    self.cleanup_and_spawn();
  }

  fn resolve_collisions(&mut self) {
    for bullet in &mut self.bullets {
      if self.arena.is_position_blocked(bullet.pos, bullet.radius) {
        bullet.collided = true;
      } else {
        for enemy in &mut self.enemies {
          if enemy.is_alive() && check_collision(bullet.pos(), bullet.shape(), enemy.pos(), enemy.shape()) {
            enemy.take_hit();
            bullet.collided = true;
          }
        }
      }
    }
  }

  fn cleanup_and_spawn(&mut self) {
    self.bullets.retain(|b| !b.should_clean());
    self.enemies.retain(|e| !e.should_clean());

    while self.enemies.len() < MIN_ENEMIES_COUNT {
      self.enemies.push(Enemy::new(self.arena.random_available_position()));
    }
  }

  pub fn render(&self) {
    let camera = self.get_camera();
    set_camera(&camera);

    clear_background(BG_COLOR);

    self.arena.render(); 
    for bullet in &self.bullets { bullet.render(); }
    for enemy in &self.enemies { enemy.render(); }
    self.player.render();

    set_default_camera();
    self.aim.render(); 
  }

  fn get_camera(&self) -> Camera2D {
    Camera2D {
      target: self.player.pos,
      zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()),
      ..Default::default()
    }
  }
}
