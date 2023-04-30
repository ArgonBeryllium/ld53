use macroquad::prelude::*;
use crate::{config::SCALE, game_objects::RenderData};

pub fn mouse_pos_scaled() -> Vec2 {
	let (x, y) = mouse_position();
	vec2(x*SCALE, y*SCALE)
}
pub fn mouse_pos_scaled_rd(rd : &RenderData) -> Vec2 {
	let (x, y) = mouse_position();
	// TODO remove; debug
	if is_key_down(KeyCode::LeftControl) { 
		return vec2(x, y) + rd.camera_offset()
	}
	vec2(x*SCALE, y*SCALE) + rd.camera_offset()
}
pub fn get_iv() -> Vec2 {
	vec2(
		if is_key_down(KeyCode::D) ||
			is_key_down(KeyCode::Right) { 1. } else { 0. } -
		if is_key_down(KeyCode::A) ||
			is_key_down(KeyCode::Left)  { 1. } else { 0. },
		if is_key_down(KeyCode::S) ||
			is_key_down(KeyCode::Down) { 1. } else { 0. } -
		if is_key_down(KeyCode::W) ||
			is_key_down(KeyCode::Up)  { 1. } else { 0. },
	)
}
pub fn get_ivn() -> Vec2 {
	let iv = get_iv();
	match iv.length().floor() as i32 {
		0 => Vec2::ZERO,
		_ => iv.normalize(),
	}
}
