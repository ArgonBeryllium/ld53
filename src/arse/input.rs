use macroquad::prelude::*;
use crate::config::SCALE;

pub fn mouse_pos_scaled() -> Vec2 {
	let (x, y) = mouse_position();
	vec2(x*SCALE, y*SCALE)
}
