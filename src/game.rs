use macroquad::prelude::*;
use macroquad::audio::{load_sound, play_sound_once, PlaySoundParams, play_sound, Sound};

use crate::traits::*;
use crate::config::*;

use crate::player::Player;
use crate::bullet::Bullet;
use crate::aim::Aim;
use crate::enemy::Enemy;
use crate::arena::Arena;
use crate::game_command::GameCommand;
use crate::hud::Hud;
use crate::cog::Cog;

pub struct Game {
  player: Player,
  aim: Aim,
  bullets: Vec<Bullet>,
  enemies: Vec<Enemy>,
  cogs: Vec<Cog>,
  arena: Arena,
  input_dir: Vec2,
  gunshot_sound: Sound,
  main_music: Sound,
  main_music_playing: bool,
  coin_sound: Sound,
}

impl Game {
  pub async fn new() -> Self {
    let arena = Arena::new();

    let gunshot_sound = load_sound("res/gunshot.wav").await.unwrap();
    let main_music = load_sound("res/main_music.wav").await.unwrap();
    let coin_sound = load_sound("res/coin.wav").await.unwrap();

    let mut world = Self {
      player: Player::new(arena.initial_player_pos),
      aim: Aim::new(),
      bullets: Vec::new(),
      enemies: Vec::new(),
      cogs: Vec::new(),
      arena,
      input_dir: Vec2::ZERO,
      gunshot_sound,
      main_music,
      main_music_playing: false,
      coin_sound,
    };

    world.spawn_initial_enemies();
    world
  }

  pub fn restart(&mut self) {
    self.player = Player::new(self.arena.initial_player_pos);
    self.bullets = Vec::new();
    self.enemies = Vec::new();
    self.cogs = Vec::new();
    self.input_dir = Vec2::ZERO;
  }

  pub fn handle_input(&mut self) -> GameCommand {
    // Handle keyboard input
    if is_key_down(KeyCode::Escape) { return GameCommand::Exit; };

    if !self.player.is_alive() {
      if is_key_down(KeyCode::Space) { return GameCommand::Restart; };
      
      return GameCommand::Continue;
    };

    let mut input_dir = Vec2::ZERO;
    if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) { input_dir.y -= 1.0; }
    if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) { input_dir.y += 1.0; }
    if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) { input_dir.x -= 1.0; }
    if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) { input_dir.x += 1.0; }
    
    self.input_dir = input_dir;

    // Handle mouse input
    let screen_mouse_pos = Vec2::from(mouse_position());
    let half_screen = vec2(screen_width() / 2.0, screen_height() / 2.0);
    let offset_from_center = screen_mouse_pos - half_screen;
    let world_mouse_pos = self.player.pos + offset_from_center;

    if is_mouse_button_pressed(MouseButton::Left) {
      self.bullets.push(Bullet::new(self.player.pos, world_mouse_pos));
      self.aim.trigger_click();

      play_sound_once(&self.gunshot_sound);
    }

    GameCommand::Continue
  }

  pub fn update(&mut self, dt: f32) {
    if !self.player.is_alive() { return };

    if !self.main_music_playing {
      play_sound(
        &self.main_music,
        PlaySoundParams { looped: true, volume: 0.3 },
      );

      self.main_music_playing = true;
    }

    self.player.update(dt);

    let sw = screen_width();
    let sh = screen_height();
      
    let player_delta = self.player.calculate_movement_delta(self.input_dir, dt);
    Self::move_and_slide(&mut self.player.pos, player_delta, PLAYER_RADIUS, &self.arena);

    let player_pos = self.player.pos;
    for enemy in &mut self.enemies {
      if !enemy.is_alive() { continue; }

      let enemy_delta = enemy.calculate_movement_delta(player_pos, dt);
      
      Self::move_and_slide(&mut enemy.pos, enemy_delta, enemy.radius, &self.arena);
    }

    self.aim.update(dt, sw, sh);
    self.bullets.update_all(dt, sw, sh);
  
    self.resolve_collisions();
    self.cleanup_and_spawn();
  }

  pub fn render(&self) {
    let camera = self.get_camera();
    set_camera(&camera);

    clear_background(BG_COLOR);

    self.arena.render(); 
    self.cogs.render_all();
    self.bullets.render_all();
    self.enemies.render_all();
    self.player.render();

    set_default_camera();

    Hud::render(self.player.life, self.player.cogs_count);
    self.aim.render(); 
  }

  fn spawn_initial_enemies(&mut self) {
    for _ in 0..4 {
      self.enemies.push(Enemy::new(self.arena.random_available_position()));
    }
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

    if self.player.is_alive() {
      for enemy in &mut self.enemies {
        if enemy.is_alive() {
          if check_collision(self.player.pos(), self.player.shape(), enemy.pos(), enemy.shape()) {
            self.player.take_hit();
          }
        }
      }

      for cog in &mut self.cogs {
        if check_collision(self.player.pos(), self.player.shape(), cog.pos(), cog.shape()) {
          self.player.pick_cog();
          cog.collided = true;
          play_sound_once(&self.coin_sound);
        }
      }
    }
  }

  fn cleanup_and_spawn(&mut self) {
    self.bullets.clean_expired();
    self.cogs.clean_expired();

    self.enemies.retain(|enemy| {
      let dead = enemy.should_clean();
    
      if dead {
        for _ in 0..rand::gen_range(1, 4) {
          self.cogs.push(Cog::new(vec2(
            rand::gen_range(enemy.pos.x - enemy.radius, enemy.pos.x + enemy.radius),
            rand::gen_range(enemy.pos.y - enemy.radius, enemy.pos.y + enemy.radius),
          )));
        }

        return false;
      } else {
        return true;
      }
    });

    while self.enemies.len() < MIN_ENEMIES_COUNT {
      self.enemies.push(Enemy::new(self.arena.random_available_position()));
    }
  }

  fn get_camera(&self) -> Camera2D {
    Camera2D {
      target: self.player.pos,
      zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()),
      ..Default::default()
    }
  }

  fn move_and_slide(pos: &mut Vec2, delta: Vec2, radius: f32, arena: &Arena) {
    let next_x = vec2(pos.x + delta.x, pos.y);
    if !arena.is_position_blocked(next_x, radius) {
      pos.x = next_x.x;
    }

    let next_y = vec2(pos.x, pos.y + delta.y);
    if !arena.is_position_blocked(next_y, radius) {
      pos.y = next_y.y;
    }
  }
}
