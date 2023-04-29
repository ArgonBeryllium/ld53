use macroquad::prelude::*;
use crate::config::SCALE;

pub fn mouse_pos_scaled() -> Vec2 {
	let (x, y) = mouse_position();
	vec2(x*SCALE, y*SCALE)
}
pub fn get_iv() -> Vec2 {
	vec2(
		if is_key_down(KeyCode::Right) { 1. } else { 0. } -
		if is_key_down(KeyCode::Left)  { 1. } else { 0. },
		if is_key_down(KeyCode::Down)  { 1. } else { 0. } -
		if is_key_down(KeyCode::Up)    { 1. } else { 0. },
	)
}
pub fn get_ivn() -> Vec2 {
	let iv = get_iv();
	match iv.length().floor() as i32 {
		0 => Vec2::ZERO,
		_ => iv.normalize(),
	}
}
