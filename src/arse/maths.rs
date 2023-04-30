use std::f32::consts::PI;

use macroquad::{prelude::{Vec2, vec2}, rand};

pub fn lerp<T:
	Copy +
	std::ops::Add<T, Output = T> +
	std::ops::Sub<T, Output = T> +
	std::ops::Mul<f32, Output = T>>
	(a : T, b : T, t : f32) -> T {
	a + (b - a)*t
}
pub fn angle_lerp(a : f32, b : f32, t : f32) -> f32 {
	if (a - b).abs() > PI
	{ lerp(a + PI*2.0, b, t) }
	else { lerp(a, b, t) }
}

pub fn random_angle() -> f32 {
	rand::gen_range(0., PI*2.0)
}
pub fn random_ring_point(
	origin : &Vec2,
	min_dist : f32,
	max_dist : f32) -> Vec2 {
	let a = random_angle();
	let d = rand::gen_range(min_dist, max_dist);
	*origin + vec2(a.cos(), a.sin())*d
}
