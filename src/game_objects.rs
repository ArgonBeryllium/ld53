use std::collections::HashMap;

use macroquad::prelude::{Vec2, vec2, is_key_down, KeyCode};
use crate::{config::{W, H}, prelude::{REAL_H, REAL_W, Assets}};

pub struct RenderData {
	pub camera_pos : Vec2,
	pub zoom : f32,
	pub assets : Option<Assets>,
	pub d : f32,
}
impl RenderData {
	pub fn new() -> Self {
		RenderData {
			camera_pos: Vec2::ZERO,
			zoom: 1.0,
			assets: None,
			d: 0.1,
		}
	}
	pub fn init(&mut self, a : &Assets) {
		self.assets = Some(a.clone());
	}
	pub fn camera_offset(&self) -> Vec2 {
		// TODO remove; debug
		if is_key_down(KeyCode::LeftControl) {
			return self.camera_pos*self.zoom - vec2(REAL_W as f32 /2., REAL_H as f32 /2.)
		}
		self.camera_pos*self.zoom - vec2(W/2., H/2.)
	}
	pub fn cast_pos(&self, p : &Vec2) -> Vec2 {
		*p*self.zoom - self.camera_offset()
	}
	pub fn scale_unit(&self, s : f32) -> f32 {
		s*self.zoom
	}
}

pub trait GameObject {
	fn init(&mut self) {}
	fn update(&mut self) -> bool { true }
	fn render(&self, rd : &RenderData);
}
pub type GameObjectID = usize;

pub struct ObjectSet<T : GameObject> {
	pub objects : HashMap<GameObjectID, T>,
	pub to_be_destroyed : Vec<GameObjectID>,
	next_id : GameObjectID,
}

#[allow(dead_code)]
impl<T : GameObject> ObjectSet<T> {
	pub fn new() -> Self {
		ObjectSet {
			objects: HashMap::new(),
			to_be_destroyed: Vec::new(),
			next_id: 0,
		}
	}
	pub fn update(&mut self) {
		for (id, obj) in self.objects.iter_mut() {
			if !obj.update() {
				self.to_be_destroyed.push(*id);
			}
		}
		for id in self.to_be_destroyed.iter() {
			self.objects.remove(&id);
		}
		self.to_be_destroyed.clear();
	}
	pub fn render(&mut self, rd : &RenderData) {
		for (_id, o) in self.objects.iter() {
			o.render(&rd);
		}
	}

	pub fn get_obj(&self, id : GameObjectID) -> &T {
		&self.objects.get(&id)
			.expect(&format!("No GameObject with id {id}"))
	}
	pub fn get_obj_mut(&mut self, id : GameObjectID) -> &mut T {
		self.objects.get_mut(&id)
			.expect(&format!("No GameObject with id {id}"))
	}

	pub fn schedule_destroy(&mut self, id : GameObjectID) {
		self.to_be_destroyed.push(id);
	}
	pub fn create(&mut self, obj : T) -> GameObjectID {
		self.objects.insert(self.next_id, obj);
		self.objects.get_mut(&self.next_id).unwrap().init();
		self.next_id += 1;
		self.next_id-1
	}
}
