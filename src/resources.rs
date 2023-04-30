use macroquad::{audio::{Sound, load_sound_from_bytes}, text::Font, prelude::*};

pub const SOUND: &[u8] = include_bytes!("../res/sound.wav");
pub const FONT_FILE: &[u8] = include_bytes!("../res/m6x11.ttf");

pub const COL_BG : Color = Color::new(0.01, 0.02, 0.04, 1.);
pub const COL_MARKER_HOME : Color = Color::new(0.67, 0.64, 0.65, 1.);
pub const COL_MARKER_FOOD : Color = Color::new(1.00, 0.51, 0.24, 1.);

pub const PARTICLE_SIZE : f32 = 6.0;

pub const TEX_FUZZY_0: &[u8] = include_bytes!("../res/fuzzy-0.png");
pub const TEX_FUZZY_1: &[u8] = include_bytes!("../res/fuzzy-1.png");
pub const TEX_FUZZY_2: &[u8] = include_bytes!("../res/fuzzy-2.png");

pub const TEX_TEST: &[u8] = include_bytes!("../res/test.png");

pub const TEX_MAP: &[u8] = include_bytes!("../res/test_map.png");
pub const TEX_VIG: &[u8] = include_bytes!("../res/vignette.png");
pub const TEX_SAND: &[u8] = include_bytes!("../res/sand.png");

pub const TEX_NEST: &[u8] = include_bytes!("../res/nest.png");
pub const TEX_NESTC: &[u8] = include_bytes!("../res/nest_col.png");

pub const TEX_SUGARS: &[&[u8]] = &[
	include_bytes!("../res/sugar_1.png"),
	include_bytes!("../res/sugar_2.png"),
	include_bytes!("../res/sugar_3.png"),
	include_bytes!("../res/sugar_4.png"),
];
pub const TEX_HIVE_FOODS: &[&[u8]] = &[
	include_bytes!("../res/hive_food_dying.png"),
	include_bytes!("../res/hive_food_bad.png"),
	include_bytes!("../res/hive_food_okay.png"),
	include_bytes!("../res/hive_food_mid.png"),
	include_bytes!("../res/hive_food_good.png"),
];

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
	pub tex_test : Texture2D,
	pub tex_map : Texture2D,
	pub tex_vig : Texture2D,
	pub tex_sand : Texture2D,
	pub tex_sugars : Vec<Texture2D>,
	pub tex_nest : Texture2D,
	pub tex_nest_col : Texture2D,
	pub tex_hive_food : Vec<Texture2D>,
}
impl Assets {
	fn load_tex_pixelated(data : &[u8]) -> Texture2D {
		let out = Texture2D::from_file_with_format(data, None);
		out.set_filter(FilterMode::Nearest);
		out
	}
	pub async fn load() -> Self {
		let mut tex_sugars = Vec::new();
		for t in TEX_SUGARS {
			tex_sugars.push(Self::load_tex_pixelated(t));
		}
		let mut tex_hive_food = Vec::new();
		for t in TEX_HIVE_FOODS {
			tex_hive_food.push(Texture2D::from_file_with_format(t, None));
		}
		Assets {
			sound: load_sound_from_bytes(SOUND).await.expect("load sound"),
			font: load_ttf_font_from_bytes(FONT_FILE).expect("load font in assets"),
			tex_fuzzy_0: Texture2D::from_file_with_format(TEX_FUZZY_0, None),
			tex_fuzzy_1: Texture2D::from_file_with_format(TEX_FUZZY_1, None),
			tex_fuzzy_2: Texture2D::from_file_with_format(TEX_FUZZY_2, None),
			tex_test: Texture2D::from_file_with_format(TEX_TEST, None),
			tex_map: Self::load_tex_pixelated(TEX_MAP),
			tex_vig: Texture2D::from_file_with_format(TEX_VIG, None),
			tex_sand: Texture2D::from_file_with_format(TEX_SAND, None),
			tex_sugars,
			tex_nest: Texture2D::from_file_with_format(TEX_NEST, None),
			tex_nest_col: Self::load_tex_pixelated(TEX_NESTC),
			tex_hive_food,
		}
	}
}
