use macroquad::audio::{play_sound, PlaySoundParams, Sound};

#[derive(Clone)]
pub struct Sfx {
  pub sound: Sound,
  pub volume: f32,
  pub looped: bool,
}

impl Sfx {
  pub fn play(&self) {
    play_sound(&self.sound, PlaySoundParams { looped: self.looped, volume: self.volume });
  }
}
