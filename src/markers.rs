use std::{collections::HashMap, cell::RefCell, rc::Rc};
use macroquad::{prelude::{Vec2, RED, vec2, WHITE}, shapes::{draw_rectangle_lines, draw_circle_lines}, texture::{draw_texture_ex, DrawTextureParams}};

use crate::{game_objects::RenderData, gobj::{FOOD_MARKER_LIFE, HOME_MARKER_LIFE, ANT_RAD, Gobj, ParticleStyle}, resources::*};

pub const MAX_HOME_MARKERS_PER_CELL : usize = 14;
pub const MARKER_RENDER_RAD : f32 = ANT_RAD*3.5;
#[derive(PartialEq, Debug, Clone)]
pub enum Marker {
	Home(Vec2, f32),
	Food(Vec2, f32)
}
impl Marker {
	pub fn pos(&self) -> &Vec2 {
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
	pub fn create_marker(&mut self, m : Marker, spawn_queue : Rc<RefCell<Vec<Gobj>>>) {
		let key = self.pos_to_key(&m.pos());
		if !self.markers.contains_key(&key) {
			self.markers.insert(key, Vec::new());
		}
		match m {
			Marker::Home(..) =>
				if self.markers.get(&key)
					.unwrap()
					.iter()
					.filter(|m| match m { Marker::Home(..) => true, _ => false })
					.count() > MAX_HOME_MARKERS_PER_CELL {
						return;
					}
			_ => ()
		}
		spawn_queue.borrow_mut().push(match m.clone() {
			Marker::Home(..) => Gobj::new_particles(
				m.pos(),
				2,
				HOME_MARKER_LIFE,
				MARKER_RENDER_RAD*0.5,
				COL_MARKER_HOME,
				ParticleStyle::Orbit(0.5, 0.03)
			),
			Marker::Food(..) => Gobj::new_particles(
				m.pos(),
				3,
				FOOD_MARKER_LIFE,
				MARKER_RENDER_RAD*0.5,
				COL_MARKER_FOOD,
				ParticleStyle::Orbit(0.9, 0.06)
			),
		});

		self.markers
			.get_mut(&key)
			.unwrap()
			.push(m);
	}
	pub fn local_markers(&self,
		pos : &Vec2,
		heading : &Vec2,
		condition : &dyn Fn(&Marker) -> bool) -> Vec<Marker> {
		let p = *pos+*heading;
		let k = self.pos_to_key(&p);

		let mut out = Vec::new();
		// every day, I am reminded of my severe mental disability
		// (the range wasn't inclusive)
		for x in (k.0-1)..=(k.0+1) {
			for y in (k.1-1)..=(k.1+1) {
				if !self.markers.contains_key(&(x, y)) {
					continue;
				}
				for m in &self.markers[&(x, y)] {
					if !condition(m) { continue; }
					out.push(m.clone());
				}
			}
		}
		out
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
	#[allow(dead_code)]
	pub fn render_debug(&self, rd : &RenderData) {
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
	#[allow(unreachable_code)]
	pub fn render(&self, rd : &RenderData) {
		return;
		for (_, v) in &self.markers {
			for p in v {
				let a = match p {
					Marker::Home(_, l) => l/HOME_MARKER_LIFE,
					Marker::Food(_, l) => l/FOOD_MARKER_LIFE,
				};
				let col = match p {
					Marker::Home(..) => COL_MARKER_HOME,
					Marker::Food(..) => COL_MARKER_FOOD,
				};
				let pos = *p.pos() - rd.camera_offset();
				draw_texture_ex(
					rd.assets.clone().unwrap().tex_fuzzy_0,
					pos.x - a*MARKER_RENDER_RAD*0.5,
					pos.y - a*MARKER_RENDER_RAD*0.5,
					col,
					DrawTextureParams {
						dest_size: Some(vec2(a, a)*MARKER_RENDER_RAD),
						..DrawTextureParams::default()
					}
				);
			}
		}
	}
}
