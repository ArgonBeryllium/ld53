use std::collections::HashMap;
use macroquad::{prelude::{Vec2, RED, vec2, WHITE}, shapes::{draw_rectangle_lines, draw_circle_lines}};

use crate::{game_objects::RenderData, gobj::{FOOD_MARKER_LIFE, HOME_MARKER_LIFE}};

#[derive(PartialEq, Debug)]
pub enum Marker {
	Home(Vec2, f32),
	Food(Vec2, f32)
}
impl Marker {
	fn pos(&self) -> &Vec2 {
		use Marker::*;
		match self {
			Home(p, _) => p,
			Food(p, _) => p,
		}
	}
	fn life(&self) -> &f32 {
		use Marker::*;
		match self {
			Home(_, l) => l,
			Food(_, l) => l,
		}
	}
	fn life_mut(&mut self) -> &mut f32 {
		use Marker::*;
		match self {
			Home(_, l) => l,
			Food(_, l) => l,
		}
	}
}
#[derive(Debug, PartialEq)]
pub struct MarkerWorld {
	grid_size : f32,
	markers : HashMap<(i32, i32), Vec<Marker>>,
}
impl MarkerWorld {
	pub fn new(grid_size : f32) -> Self {
		MarkerWorld {
			grid_size,
			markers: HashMap::new()
		}
	}
	pub fn pos_to_key(&self, pos : &Vec2) -> (i32, i32) {
		let key = *pos / self.grid_size;
		(key.x.floor() as i32, key.y.floor() as i32)
	}
	pub fn create_marker(&mut self, m : Marker) {
		let key = self.pos_to_key(&m.pos());
		if !self.markers.contains_key(&key) {
			self.markers.insert(key, Vec::new());
		}
		self.markers
			.get_mut(&key)
			.unwrap()
			.push(m);
	}
	pub fn detect_marker(&self, pos : &Vec2, heading : &Vec2) -> Option<&Marker> {
		let p = *pos+*heading;
		let k = self.pos_to_key(&p);

		let (mut winner, mut md) = (None, f32::MAX);
		for x in (k.0-1)..(k.0+1) {
			for y in (k.1-1)..(k.1+1) {
				if !self.markers.contains_key(&(x, y)) {
					continue;
				}
				for m in &self.markers[&(x, y)] {
					let d = m.pos().distance(p);
					if d < md {
						winner = Some(m);
						md = d;
					}
				}
			}
		}
		winner
	}

	pub fn update(&mut self, d : f32) {
		self.markers.retain(|_, v| {
			let mut i = 0;
			while i < v.len() {
				*v[i].life_mut() -= d;
				if *v[i].life() < 0.0 {
					v.remove(i);
				} else { i += 1 }
			}
			i != 0
		});
	}
	pub fn render(&self, rd : &RenderData) {
		for (k, v) in &self.markers {
			let pos = vec2(
				k.0 as f32 * self.grid_size,
				k.1 as f32 * self.grid_size
			);
			let pos = pos - rd.camera_offset();
			draw_rectangle_lines(pos.x, pos.y, self.grid_size, self.grid_size, 2., RED);
			for p in v {
				let a = match p {
					Marker::Home(_, l) => l/HOME_MARKER_LIFE,
					Marker::Food(_, l) => l/FOOD_MARKER_LIFE,
				};
				let col = match p {
					Marker::Home(..) => WHITE,
					Marker::Food(..) => RED,
				};
				let pos = *p.pos() - rd.camera_offset();
				draw_circle_lines(pos.x, pos.y, a*2., 1.4, col);
			}
		}
	}
}
