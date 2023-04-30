use macroquad::{audio::{Sound, load_sound_from_bytes}, text::Font, prelude::*};

pub const SOUND: &[u8] = include_bytes!("../res/sound.wav");
pub const FONT_FILE: &[u8] = include_bytes!("../res/m6x11.ttf");

pub const COL_BG : Color = Color::new(0.01, 0.02, 0.04, 1.);
pub const COL_MARKER_HOME : Color = Color::new(0.97, 0.94, 0.95, 1.);
pub const COL_MARKER_FOOD : Color = Color::new(0.95, 0.59, 0.24, 1.);

pub const PARTICLE_SIZE : f32 = 6.0;

pub const TEX_FUZZY_0: &[u8] = include_bytes!("../res/fuzzy-0.png");
pub const TEX_FUZZY_1: &[u8] = include_bytes!("../res/fuzzy-1.png");
pub const TEX_FUZZY_2: &[u8] = include_bytes!("../res/fuzzy-2.png");

pub fn load_font() -> Font {
	load_ttf_font_from_bytes(FONT_FILE).expect("load font fun")
}

#[derive(Clone)]
pub struct Assets {
	pub sound : Sound,
	pub font : Font,
	pub tex_fuzzy_0 : Texture2D,
	pub tex_fuzzy_1 : Texture2D,
	pub tex_fuzzy_2 : Texture2D,
}
impl Assets {
	pub async fn load() -> Self {
		Assets {
			sound: load_sound_from_bytes(SOUND).await.expect("load sound"),
			font: load_ttf_font_from_bytes(FONT_FILE).expect("load font in assets"),
			tex_fuzzy_0: Texture2D::from_file_with_format(TEX_FUZZY_1, None),
			tex_fuzzy_1: Texture2D::from_file_with_format(TEX_FUZZY_1, None),
			tex_fuzzy_2: Texture2D::from_file_with_format(TEX_FUZZY_2, None),
		}
	}
}
