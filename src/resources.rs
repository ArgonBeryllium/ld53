use macroquad::{audio::{Sound, load_sound_from_bytes}, text::Font, prelude::*};

pub const SOUND: &[u8] = include_bytes!("../res/sound.wav");
pub const FONT_FILE: &[u8] = include_bytes!("../res/m6x11.ttf");

#[derive(Clone)]
pub struct Assets {
	pub sound : Sound,
	pub font : Font,
}
impl Assets {
	pub async fn load() -> Self {
		Assets {
			sound: load_sound_from_bytes(SOUND).await.expect("load sound"),
			font: load_ttf_font_from_bytes(FONT_FILE).expect("load font"),
		}
	}
}
