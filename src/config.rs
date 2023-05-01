use macroquad::prelude::*;

pub const REAL_W : i32 = 800;
pub const REAL_H : i32 = 600;
pub const SCALE : f32 = 0.2;
pub const W : f32 = REAL_W as f32 * SCALE;
pub const H : f32 = REAL_H as f32 * SCALE;

pub const DEFAULT_FONT_SIZE : f32 = 8.;

pub fn window_conf() -> Conf {
	Conf {
		window_width: REAL_W,
		window_height: REAL_H,
		window_title: "HIVE".to_owned(),
		fullscreen: false,
		window_resizable: false,
		..Conf::default()
	}
}
