use macroquad::prelude::*;

mod traits;
mod player;
mod bullet;
mod aim;
mod enemy;
mod game;
mod game_command;
mod config;
mod wall;
mod floor;
mod arena;
mod alt_shapes;
mod hud;
mod cog;

use crate::game::Game;
use crate::game_command::GameCommand;

fn window_conf() -> Conf {
  Conf {
    window_title: "Blood & Cog".to_owned(),
    window_width: 1280,
    window_height: 720,
    fullscreen: false,
    window_resizable: true,
    ..Default::default()
  }
}

#[macroquad::main(window_conf)]
async fn main() {
  show_mouse(false);

  macroquad::rand::srand(miniquad::date::now() as u64);

  let mut game = Game::new().await;

  loop {
    let dt = get_frame_time();

    match game.handle_input() {
      GameCommand::Exit => break,
      GameCommand::Continue => {
        game.update(dt);
        game.render();
      }
      GameCommand::Restart => {
        game.restart();
      }
    }

    next_frame().await;
  }
}
