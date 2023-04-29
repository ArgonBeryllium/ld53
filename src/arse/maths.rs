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

pub fn random_ring_point(
	origin : &Vec2,
	min_dist : f32,
	max_dist : f32) -> Vec2 {
	let a = rand::gen_range(0., PI*2.0);
	let d = rand::gen_range(min_dist, max_dist);
	*origin + vec2(a.cos(), a.sin())*d
}
