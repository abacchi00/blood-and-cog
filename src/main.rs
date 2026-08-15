use macroquad::prelude::*;

mod player;
mod direction;
mod bullet;
mod aim;

use crate::player::Player;
use crate::direction::Direction;
use crate::bullet::Bullet;
use crate::aim::Aim;

#[macroquad::main("Blood & Cog")]
async fn main() {
  show_mouse(false);

  let mut player = Player::new();
  let mut aim = Aim::new();
  let mut bullets: Vec<Bullet> = Vec::new();

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

      bullet.pos.x >= 0.0 
          && bullet.pos.x <= screen_width() 
          && bullet.pos.y >= 0.0 
          && bullet.pos.y <= screen_height()
    });

    player.render();
    aim.render(mouse_x, mouse_y);

    next_frame().await;
  }
}