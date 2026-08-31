use macroquad::prelude::*;
use macroquad::audio::{load_sound, play_sound_once, PlaySoundParams, play_sound, Sound};

use crate::traits::*;
use crate::config::*;

use crate::player::Player;
use crate::bullet::Bullet;
use crate::aim::Aim;
use crate::scrawler::Scrawler;
use crate::arena::Arena;
use crate::game_command::GameCommand;
use crate::hud::Hud;
use crate::cog::Cog;
use crate::leechy::Leechy;
use crate::blood_pool::BloodPool;

pub struct Game {
  started: bool,
  player: Player,
  aim: Aim,
  bullets: Vec<Bullet>,
  scrawlers: Vec<Scrawler>,
  leechies: Vec<Leechy>, 
  cogs: Vec<Cog>,
  blood_pools: Vec<BloodPool>,
  arena: Arena,
  input_dir: Vec2,
  gunshot_sound: Sound,
  main_music: Sound,
  coin_sound: Sound,
  scrawler_injured_sound: Sound,
  scrawler_dying_sound: Sound,
  leechy_injured_sound: Sound,
  leechy_dying_sound: Sound,
  hud: Hud,
  zoom_level: f32,
  leechies_kill_count: i32,
  scrawlers_kill_count: i32,
  screen_shake: f32,
}

impl Game {
  pub async fn new() -> Self {
    let arena = Arena::new();

    let gunshot_sound = load_sound("res/gunshot.wav").await.unwrap();
    let main_music = load_sound("res/main_music.wav").await.unwrap();
    let coin_sound = load_sound("res/coin.wav").await.unwrap();
    let scrawler_injured_sound = load_sound("res/scrawler_injured.wav").await.unwrap();
    let scrawler_dying_sound = load_sound("res/scrawler_dying.wav").await.unwrap();
    let leechy_injured_sound = load_sound("res/leechy_injured.ogg").await.unwrap();
    let leechy_dying_sound = load_sound("res/leechy_dying.wav").await.unwrap();

    let mut world = Self {
      started: false,
      player: Player::new(arena.initial_player_pos),
      aim: Aim::new(),
      bullets: Vec::new(),
      scrawlers: Vec::new(),
      leechies: Vec::new(),
      blood_pools: Vec::new(),
      cogs: Vec::new(),
      arena,
      input_dir: Vec2::ZERO,
      gunshot_sound,
      main_music,
      coin_sound,
      scrawler_injured_sound,
      scrawler_dying_sound,
      leechy_injured_sound,
      leechy_dying_sound,
      hud: Hud::new(),
      zoom_level: 2.0,
      leechies_kill_count: 0,
      scrawlers_kill_count: 0,
      screen_shake: 0.0,
    };

    play_sound(
      &world.main_music,
      PlaySoundParams { looped: true, volume: 0.3 },
    );

    world.spawn_arena_enemies();
    world
  }

  pub fn start(&mut self) {
    self.started = true;
  }

  pub fn restart(&mut self) {
    self.player = Player::new(self.arena.initial_player_pos);
    self.bullets = Vec::new();
    self.scrawlers = Vec::new();
    self.leechies = Vec::new();
    self.blood_pools = Vec::new();
    self.cogs = Vec::new();
    self.input_dir = Vec2::ZERO;
    self.scrawlers_kill_count = 0;
    self.leechies_kill_count = 0;
  }

  pub fn handle_input(&mut self) -> GameCommand {
    // Handle keyboard input
    if is_key_down(KeyCode::Escape) { return GameCommand::Exit; };

    if !self.started {
      if is_key_down(KeyCode::Space) { return GameCommand::Start; };

      return GameCommand::Halt;
    }

    if !self.player.is_alive() {
      if is_key_down(KeyCode::Space) { return GameCommand::Restart; };
      
      return GameCommand::Halt;
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
      let spawn_pos = self.player.get_barrel_tip_pos();
      self.bullets.push(Bullet::new(spawn_pos, world_mouse_pos));
      self.aim.trigger_click();
      self.player.trigger_recoil();
      self.screen_shake = 3.0;
      play_sound_once(&self.gunshot_sound);
    }

    // Handle mouse wheel zoom
    let (_, wheel_y) = mouse_wheel();
    if wheel_y != 0.0 {
      self.zoom_level += wheel_y * 0.1;
      self.zoom_level = self.zoom_level.clamp(1.0, 5.0);
    }

    GameCommand::Continue
  }

  pub fn update(&mut self, dt: f32) {
    if self.screen_shake > 0.0 {
      self.screen_shake -= dt * 60.0;
      self.screen_shake = self.screen_shake.max(0.0);
    }

    self.player.update(dt);

    let sw = screen_width();
    let sh = screen_height();
      
    let player_delta = self.player.calculate_movement_delta(self.input_dir, dt);
    Self::move_and_slide(&mut self.player.pos, player_delta, PLAYER_RADIUS, &self.arena);

    let player_pos = self.player.pos();
    
    // Updates Scrawlers
    for scrawler in &mut self.scrawlers {
      if !scrawler.is_alive() { continue; }
      let scrawler_delta = scrawler.update(player_pos, dt);
      Self::move_and_slide(&mut scrawler.pos, scrawler_delta, scrawler.radius, &self.arena);
    }

    // Updates Leechies
    for leechy in &mut self.leechies {
      if !leechy.is_alive() { continue; }
      let (leechy_delta, spawned_blood) = leechy.update(player_pos, dt);
      Self::move_and_slide(&mut leechy.pos, leechy_delta, leechy.radius, &self.arena);
      if let Some(pos) = spawned_blood {
        self.blood_pools.push(BloodPool::new(pos));
      }
    }

    // Updates BloodPools
    for blood in &mut self.blood_pools {
      blood.update(dt);
    }

    self.aim.update(dt, sw, sh);
    self.bullets.update_all(dt, sw, sh);
    self.hud.update(dt, sw, sh);

    self.resolve_collisions();
    self.cleanup_and_spawn();
  }

  pub fn render(&self) {
    let camera = self.get_camera();
    set_camera(&camera);

    clear_background(BG_COLOR);

    self.arena.render(&self.blood_pools); 
    self.cogs.render_all();
    self.bullets.render_all();
    self.scrawlers.render_all();
    self.leechies.render_all();
    self.player.render();

    set_default_camera();

    self.hud.render(self.player.hp.current, self.player.cogs_count, self.started, self.leechies_kill_count, self.scrawlers_kill_count);
    self.aim.render(); 
  }

  fn resolve_collisions(&mut self) {
    for bullet in &mut self.bullets {
      if self.arena.is_position_blocked(bullet.pos, bullet.radius) {
        bullet.collided = true;
      } else {
        // Bullets collisions with Scrawlers
        for scrawler in &mut self.scrawlers {
          if scrawler.is_alive() && check_collision(bullet.pos(), bullet.shape(), scrawler.pos(), scrawler.shape()) {
            scrawler.take_hit();
            bullet.collided = true;
            play_sound(&self.scrawler_injured_sound, PlaySoundParams { looped: false, volume: 0.08 });
          }
        }

        // Bullets collisions with Leechies
        for leechy in &mut self.leechies {
          if leechy.is_alive() && check_collision(bullet.pos(), bullet.shape(), leechy.pos(), leechy.shape()) {
            leechy.take_hit();
            bullet.collided = true;
            play_sound(&self.leechy_injured_sound, PlaySoundParams { looped: false, volume: 0.4 });
          }
        }
      }
    }

    if self.player.is_alive() {
      // Player collision with Scrawlers
      for scrawler in &mut self.scrawlers {
        if scrawler.is_alive() {
          if check_collision(self.player.pos(), self.player.shape(), scrawler.pos(), scrawler.shape()) {
            self.player.take_hit();
          }
        }
      }

      // Player collision with Leechies
      for leechy in &mut self.leechies {
        if leechy.is_alive() {
          if check_collision(self.player.pos(), self.player.shape(), leechy.pos(), leechy.shape()) {
            self.player.take_hit();
          }
        }
      }

      // Cogs collecting
      for cog in &mut self.cogs {
        if check_collision(self.player.pos(), self.player.shape(), cog.pos(), cog.shape()) {
          self.player.pick_cog();
          cog.mark_as_collected();
          self.hud.display_cog_collected();
          play_sound_once(&self.coin_sound);
        }
      }
    }
  }

  fn cleanup_and_spawn(&mut self) {
    self.bullets.clean_expired();
    self.cogs.clean_expired();
    self.blood_pools.clean_expired();

    // Scrawlers cleanup and drop
    self.scrawlers.retain(|scrawler| {
      let dead = scrawler.should_clean();

      if dead {
        self.scrawlers_kill_count += 1;

        for _ in 0..rand::gen_range(SCRAWLER_COG_DROP_QUANT_MIN, SCRAWLER_COG_DROP_QUANT_MAX + 1) {
          self.cogs.push(Cog::new(vec2(
            rand::gen_range(scrawler.pos.x - scrawler.radius, scrawler.pos.x + scrawler.radius),
            rand::gen_range(scrawler.pos.y - scrawler.radius, scrawler.pos.y + scrawler.radius),
          )));
        }

        play_sound(&self.scrawler_dying_sound, PlaySoundParams { looped: false, volume: 0.7 });

        return false;
      } else {
        return true;
      }
    });

    // Leechies cleanup and drop
    self.leechies.retain(|leechy| {
      let dead = leechy.should_clean();

      if dead {
        self.leechies_kill_count += 1;
        
        for _ in 0..rand::gen_range(LEECHY_BLOOD_POOL_DROP_QUANT_MIN, LEECHY_BLOOD_POOL_DROP_QUANT_MAX + 1) {
          self.blood_pools.push(BloodPool::new(vec2(
            rand::gen_range(leechy.pos.x - leechy.radius * LEECHY_BLOOD_POOL_DROP_SPREAD, leechy.pos.x + leechy.radius * LEECHY_BLOOD_POOL_DROP_SPREAD),
            rand::gen_range(leechy.pos.y - leechy.radius * LEECHY_BLOOD_POOL_DROP_SPREAD, leechy.pos.y + leechy.radius * LEECHY_BLOOD_POOL_DROP_SPREAD),
          )));
        }

        play_sound(&self.leechy_dying_sound, PlaySoundParams { looped: false, volume: 3.0 });

        return false;
      } else {
        return true;
      }
    });

    self.spawn_arena_enemies();
  }

  fn get_camera(&self) -> Camera2D {
    let mut camera_target = self.player.pos;

    if self.screen_shake > 0.0 {
      let offset_x = macroquad::rand::gen_range(-1.0, 1.0) * self.screen_shake;
      let offset_y = macroquad::rand::gen_range(-1.0, 1.0) * self.screen_shake;
      camera_target += vec2(offset_x, offset_y);
    }

    Camera2D {
      target: camera_target,
      zoom: vec2(self.zoom_level / screen_width(), self.zoom_level / screen_height()),
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

  fn spawn_arena_enemies(&mut self) {
    for _ in self.scrawlers.len()..ARENA_MIN_ENEMIES_COUNT {
      self.scrawlers.push(Scrawler::new(self.arena.random_available_position()));
    }

    for _ in self.leechies.len()..ARENA_MIN_ENEMIES_COUNT {
      self.leechies.push(Leechy::new(self.arena.random_available_position()));
    }
  }
}
