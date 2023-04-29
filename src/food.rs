use macroquad::prelude::*;
use std::collections::HashMap;

use crate::game_objects::RenderData;

#[derive(Debug, PartialEq)]
pub struct Food {
	pos : Vec2
}
impl Food {
	pub fn new(pos : &Vec2) -> Self {
		Food {
			pos: *pos
		}
	}
	fn render(&self, rd : &RenderData) {
		let pos = self.pos - rd.camera_offset();
		draw_rectangle(pos.x, pos.y, 5., 5., GREEN)
	}
}

#[derive(Debug, PartialEq)]
pub struct FoodWorld {
	grid_size : f32,
	food : HashMap<(i32, i32), Vec<Food>>,
}
impl FoodWorld {
	pub fn new(grid_size : f32) -> Self {
		FoodWorld {
			grid_size,
			food: HashMap::new()
		}
	}
	pub fn pos_to_key(&self, pos : &Vec2) -> (i32, i32) {
		let key = *pos / self.grid_size;
		(key.x.floor() as i32, key.y.floor() as i32)
	}
	pub fn put_food(&mut self, f : Food) {
		let key = self.pos_to_key(&f.pos);
		if !self.food.contains_key(&key) {
			self.food.insert(key, Vec::new());
		}
		self.food
			.get_mut(&key)
			.unwrap()
			.push(f);
	}
	pub fn find_food(&self, pos : &Vec2, heading : &Vec2) -> Option<&Food> {
		let p = *pos+*heading;
		let k = self.pos_to_key(&p);

		let (mut winner, mut md) = (None, f32::MAX);
		for x in (k.0-1)..(k.0+1) {
			for y in (k.1-1)..(k.1+1) {
				if !self.food.contains_key(&(x, y)) {
					continue;
				}
				for m in &self.food[&(x, y)] {
					let d = m.pos.distance(p);
					if d < md {
						winner = Some(m);
						md = d;
					}
				}
			}
		}
		winner
	}
	pub fn take_food(&mut self, f : &Food) -> Option<Food> {
		for (_, v) in self.food.iter_mut() {
			let i = 0;
			while i < v.len() {
				if v[i] == *f {
					return Some(v.remove(i));
				}
			}
		}
		None
	}

	pub fn update(&mut self, d : f32) {
		self.food.retain(|_, v| v.len() != 0);
	}
	pub fn render(&self, rd : &RenderData) {
		for (k, v) in &self.food {
			let pos = vec2(
				k.0 as f32 * self.grid_size,
				k.1 as f32 * self.grid_size
			);
			let pos = pos - rd.camera_offset();
			draw_rectangle_lines(pos.x, pos.y, self.grid_size, self.grid_size, 2., PINK);
			for f in v {
				f.render(rd);
			}
		}
	}
}
