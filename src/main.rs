use macroquad::prelude::*;

mod player;
mod direction;
mod bullet;
mod aim;
mod enemy;

use crate::player::Player;
use crate::direction::Direction;
use crate::bullet::Bullet;
use crate::aim::Aim;
use crate::enemy::Enemy;

fn bullet_collision(a: &Bullet, b: &Enemy) -> bool {
  let dx = a.pos.x - b.pos.x;
  let dy = a.pos.y - b.pos.y;
  
  let c_sqrd = (dx * dx) + (dy * dy);
  
  let radius_sum = a.radius + b.radius;
  let radius_sum_sqrd = radius_sum * radius_sum;
  
  c_sqrd < radius_sum_sqrd
}

#[macroquad::main("Blood & Cog")]
async fn main() {
  show_mouse(false);

  let mut player = Player::new();
  let mut aim = Aim::new();
  let mut bullets: Vec<Bullet> = Vec::new();
  let mut enemies: Vec<Enemy> = Vec::new();

  enemies.push(Enemy::new(Vec2::new(player.pos.x - screen_width()/3.0, player.pos.y - screen_height()/3.0)));
  enemies.push(Enemy::new(Vec2::new(player.pos.x - screen_width()/3.0, player.pos.y + screen_height()/3.0)));
  enemies.push(Enemy::new(Vec2::new(player.pos.x + screen_width()/3.0, player.pos.y - screen_height()/3.0)));
  enemies.push(Enemy::new(Vec2::new(player.pos.x + screen_width()/3.0, player.pos.y + screen_height()/3.0)));

  loop {
    clear_background(BLACK);

    let dt = get_frame_time();

    player.change_dirs(Direction::current_directions());
    player.update_pos(dt);

    let (mouse_x, mouse_y) = mouse_position();
    let mouse_pos = Vec2::new(mouse_x, mouse_y);

    if is_mouse_button_pressed(MouseButton::Left) {
      bullets.push(Bullet::new(player.pos, mouse_pos));
      aim.trigger_click();
    }

    aim.update(dt);

    bullets.retain_mut(|bullet| {
      bullet.update_pos(dt);
      bullet.render();
      
      let mut hit_enemy = false;

      enemies.retain_mut(|enemy| {
        if bullet_collision(bullet, enemy) {
          let dead = enemy.take_hit();

          hit_enemy = true;

          if dead { return false; }
        }

        true
      });

      bullet.pos.x >= 0.0 
        && bullet.pos.x <= screen_width() 
        && bullet.pos.y >= 0.0 
        && bullet.pos.y <= screen_height()
        && !hit_enemy
    });

    enemies.iter().for_each(|enemy| enemy.render());

    player.render();
    aim.render(mouse_x, mouse_y);

    next_frame().await;
  }
}