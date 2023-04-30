use macroquad::prelude::*;
use std::collections::HashMap;

use crate::{game_objects::RenderData, prelude::{COL_MARKER_FOOD, random_angle}, gobj::PLAYER_RAD};

#[derive(Debug, PartialEq, Clone)]
pub struct Food {
	pub pos : Vec2,
	pub size : f32,
	pub value : f32,
	pub angle : f32,
	pub index : usize,
}
impl Food {
	pub fn new(pos : &Vec2) -> Self {
		let value = rand::gen_range(1., 5.);
		let size = value*PLAYER_RAD;
		Food {
			pos: *pos,
			size,
			value,
			angle: random_angle(),
			index: rand::gen_range(0, 4),
		}
	}
	pub fn render(&self, rd : &RenderData) {
		let pos = rd.cast_pos(&self.pos);
		let dim = Vec2::ONE*rd.scale_unit(self.size);
		draw_texture_ex(rd.assets.clone().unwrap().tex_sugars[self.index],
			pos.x - dim.x/2., pos.y - dim.y/2.,
			COL_MARKER_FOOD,
			DrawTextureParams {
				dest_size: Some(dim),
				rotation: self.angle,
				..DrawTextureParams::default()
			})
	}
}

pub type FoodID = usize;
#[derive(Debug)]
pub struct FoodWorld {
	grid_size : f32,
	food : HashMap<(i32, i32), HashMap<FoodID, Food>>,
	next_id : FoodID,
}
impl FoodWorld {
	pub fn new(grid_size : f32) -> Self {
		FoodWorld {
			grid_size,
			food: HashMap::new(),
			next_id: 0,
		}
	}
	pub fn pos_to_key(&self, pos : &Vec2) -> (i32, i32) {
		let key = *pos / self.grid_size;
		(key.x.floor() as i32, key.y.floor() as i32)
	}
	pub fn put_food(&mut self, f : Food) {
		let key = self.pos_to_key(&f.pos);
		if !self.food.contains_key(&key) {
			self.food.insert(key, HashMap::new());
		}
		self.next_id += 1;
		self.food
			.get_mut(&key)
			.unwrap()
			.insert(self.next_id, f);
	}
	pub fn find_food(&self, pos : &Vec2, heading : &Vec2) -> Option<FoodID> {
		let p = *pos+*heading;
		let k = self.pos_to_key(&p);

		let (mut winner, mut md) = (None, f32::MAX);
		for x in (k.0-1)..=(k.0+1) {
			for y in (k.1-1)..=(k.1+1) {
				if !self.food.contains_key(&(x, y)) {
					continue;
				}
				for m in &self.food[&(x, y)] {
					let d = m.1.pos.distance(p);
					if d < md {
						winner = Some(*m.0);
						md = d;
					}
				}
			}
		}
		winner
	}
	pub fn get_food(&self, food_id : FoodID) -> Option<&Food> {
		for (_, v) in self.food.iter() {
			if v.contains_key(&food_id) {
				return v.get(&food_id)
			}
		}
		None
	}
	pub fn take_food(&mut self, food_id : FoodID) -> Option<Food> {
		for (_, v) in self.food.iter_mut() {
			if v.contains_key(&food_id) {
				return v.remove(&food_id)
			}
		}
		None
	}

	pub fn update(&mut self, _d : f32) {
		self.food.retain(|_, v| v.len() != 0);
	}
	pub fn render(&self, rd : &RenderData) {
		for (_k, v) in &self.food {
			//let pos = vec2(
			//	k.0 as f32 * self.grid_size,
			//	k.1 as f32 * self.grid_size
			//);
			//let pos = pos - rd.camera_offset();
			//draw_rectangle_lines(pos.x, pos.y, self.grid_size, self.grid_size, 2., PINK);
			for f in v {
				f.1.render(rd);
			}
		}
	}
}
