use std::collections::HashMap;

use macroquad::prelude::{Vec2, vec2};

use crate::{food::FoodWorld, markers::MarkerWorld, prelude::Assets};

pub const COLLISION_GRID_SIZE_TEXPIXELS : f32 = 4.;
const MAP_SIZE : f32 = 1500.;
pub const MAP_DIMS : Vec2 = Vec2::new(MAP_SIZE, MAP_SIZE);
pub const MAP_TOPLEFT : Vec2 = Vec2::new(-MAP_DIMS.x/2., -MAP_DIMS.y/2.);
#[derive(Debug)]
pub struct World {
	pub food : FoodWorld,
	pub marker : MarkerWorld,
	pub collision_map : HashMap<(i32, i32), bool>,
	pub collision_cell_size : Vec2,
}
pub const FOOD_WORLD_GRID_SIZE : f32 = 42.;
pub const MARK_WORLD_GRID_SIZE : f32 = 55.;
impl World {
	pub fn new() -> Self {
		World {
			food: FoodWorld::new(FOOD_WORLD_GRID_SIZE),
			marker: MarkerWorld::new(MARK_WORLD_GRID_SIZE),
			collision_map: HashMap::new(),
			collision_cell_size: Vec2::ZERO,
		}
	}
	pub fn init(&mut self, assets : &Assets) {
		let td = assets.tex_map.get_texture_data();

		let w = assets.tex_map.width()/COLLISION_GRID_SIZE_TEXPIXELS;
		let h = assets.tex_map.height()/COLLISION_GRID_SIZE_TEXPIXELS;
		self.collision_cell_size = MAP_DIMS / vec2(w, h);

		for x in 0..(w as i32) {
			for y in 0..(h as i32) {
				self.collision_map.insert((x, y),
				td.get_pixel(
					((x as f32 + 0.5) * COLLISION_GRID_SIZE_TEXPIXELS) as u32,
					((y as f32 + 0.5) * COLLISION_GRID_SIZE_TEXPIXELS) as u32
				).r > 0.19);
			}
		}
	}
	pub fn is_collision(&self, pos : &Vec2) -> bool {
		let key = (*pos - MAP_TOPLEFT)/self.collision_cell_size;
		let key = (key.x.floor() as i32, key.y.floor() as i32);
		if !self.collision_map.contains_key(&key) { true }
		else { *self.collision_map.get(&key).unwrap() }
	}
}
