use macroquad::prelude::*;

mod traits;
mod player;
mod direction;
mod bullet;
mod aim;
mod enemy;
mod game_world;
mod config;

use crate::game_world::GameWorld;

#[macroquad::main("Blood & Cog")]
async fn main() {
  show_mouse(false);
  let mut game = GameWorld::new();

  loop {
    let dt = get_frame_time();

    game.handle_input();
    game.update(dt);
    game.render();

    next_frame().await;
  }
}
