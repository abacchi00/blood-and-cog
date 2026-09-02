use macroquad::audio::{load_sound};

use crate::sfx::Sfx;

pub struct GameAudio {
  pub main_music: Sfx,
  pub gunshot: Sfx,
  pub coin: Sfx,
  pub scrawler_injured: Sfx,
  pub scrawler_dying: Sfx,
  pub leechy_injured: Sfx,
  pub leechy_dying: Sfx,
}

impl GameAudio {
  pub async fn load() -> Self {
    let main_music = load_sound("res/main_music.wav").await.unwrap();
    let gunshot_sound = load_sound("res/gunshot.wav").await.unwrap();
    let coin_sound = load_sound("res/coin.wav").await.unwrap();
    let scrawler_injured_sound = load_sound("res/scrawler_injured.wav").await.unwrap();
    let scrawler_dying_sound = load_sound("res/scrawler_dying.wav").await.unwrap();
    let leechy_injured_sound = load_sound("res/leechy_injured.ogg").await.unwrap();
    let leechy_dying_sound = load_sound("res/leechy_dying.wav").await.unwrap();

    Self {
      main_music: Sfx { sound: main_music, volume: 0.3, looped: true },
      gunshot: Sfx { sound: gunshot_sound, volume: 1.0, looped: false },
      coin: Sfx { sound: coin_sound, volume: 1.0, looped: false },
      scrawler_injured: Sfx { sound: scrawler_injured_sound, volume: 0.08 , looped: false },
      scrawler_dying: Sfx { sound: scrawler_dying_sound, volume: 0.7, looped: false },
      leechy_injured: Sfx { sound: leechy_injured_sound, volume: 0.4, looped: false },
      leechy_dying: Sfx { sound: leechy_dying_sound, volume: 3.0, looped: false },
    }
  }
}
