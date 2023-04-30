use crate::Scene;
use crate::gobj::PLAYER_RAD;
use std::{rc::Rc, cell::RefCell};
use crate::*;

use crate::{RenderData, game_objects::ObjectSet, gobj::Gobj, world::World, hive::Hive};

pub struct Nest {
	rd : RenderData,
	objs : ObjectSet<Gobj>,
	sq : Rc<RefCell<Vec<Gobj>>>,
	world : Rc<RefCell<World>>,
}
impl Nest {
	pub fn new(hive : Rc<RefCell<Hive>>) -> Self {
		Nest {
			rd: RenderData::new(),
			objs: ObjectSet::new(),
			sq: Rc::new(RefCell::new(Vec::new())),
			world: Rc::new(RefCell::new(World::new(hive))),
		}
	}
	fn render_map_vignette(&self, tl : &Vec2, map_dims : &Vec2, scale_factor : f32) {
		let map_vignette_dims = *map_dims*self.rd.scale_unit(scale_factor);
		let map_vignette_pos = self.rd.cast_pos(&(
				*tl + *map_dims/2.
				- *map_dims*scale_factor/2.
				)
			);
		draw_texture_ex(self.rd.assets.clone().unwrap().tex_vig, map_vignette_pos.x, map_vignette_pos.y, BLACK,
			DrawTextureParams {
				dest_size: Some(map_vignette_dims),
				..DrawTextureParams::default()
			});
	}
	fn render_debug_map_col(&self) {
		let css = self.world.borrow().collision_cell_size*self.rd.scale_unit(1.);
		for (p, b) in self.world.borrow().collision_map.iter() {
			let p = self.rd.cast_pos(
				&(vec2(p.0 as f32, p.1 as f32)
				* css
				+ NEST_TOPLEFT));
			draw_rectangle_lines(p.x, p.y,
				css.x,
				css.y, 1.0, RED);
			if *b {
				draw_rectangle(p.x, p.y,
					css.x,
					css.y, RED);
			}
		}
	}
	pub fn player_pos(&self) -> Vec2 {
		match self.objs.get_obj(0) {
			Gobj::Player(_, _, pos, _, _, _) => *pos,
			_ => panic!("player_id ({}) not pointing to a Player!", 0)
		}
	}
	fn set_player_pos(&mut self, p : &Vec2) {
		match self.objs.get_obj_mut(0) {
			Gobj::Player(_, _, pos, _, _, _) => *pos = *p,
			_ => panic!("player_id ({}) not pointing to a Player!", 0)
		}
	}
}
const NEST_SIZE : f32 = 500.;
pub const NEST_DIMS : Vec2 = Vec2::new(NEST_SIZE, NEST_SIZE);
pub const NEST_TOPLEFT : Vec2 = Vec2::new(-NEST_DIMS.x/2., -NEST_DIMS.y/2.);
impl Scene for Nest {
	fn init(&mut self, a : &Assets) {
		self.rd.assets = Some(a.clone());
		self.rd.zoom = 0.8;

		self.world.borrow_mut().map_tl = NEST_TOPLEFT;
		self.world.borrow_mut().init(&a.tex_nest_col, &NEST_DIMS);

		self.objs.create(Gobj::new_player(self.sq.clone(), self.world.clone(), &vec2(0., NEST_TOPLEFT.y+NEST_DIMS.y*0.6)));
	}
	fn load(&mut self) {
		self.set_player_pos(&vec2(0., NEST_TOPLEFT.y+NEST_DIMS.y*0.7));
		self.objs.create(Gobj::Fader(1.0));
	}
    fn update(&mut self, q : &mut SignalQueue) {
		self.objs.update();
		self.sq.borrow_mut().clear();

		if self.player_pos().y > NEST_TOPLEFT.y + NEST_DIMS.y - PLAYER_RAD*20. {
			q.send(Signal::SetScene(1));
		}
    }

    fn render(&mut self, _q : &mut SignalQueue) {
		let d = get_frame_time().min(0.2).max(0.001);
		self.rd.camera_pos = lerp(
			self.rd.camera_pos,
			self.player_pos()*0.6,
			d*3.);
		self.rd.zoom = lerp(
			self.rd.zoom, 
			if self.player_pos().y < PLAYER_RAD*18. {
				0.3
			} else { 1.0 },
			d);

		let mapp = self.rd.cast_pos(&NEST_TOPLEFT);
		draw_texture_ex(self.rd.assets.clone().unwrap().tex_nest, mapp.x, mapp.y, WHITE,
			DrawTextureParams {
				dest_size: Some(NEST_DIMS*self.rd.scale_unit(1.)),
				..DrawTextureParams::default()
			});
		draw_texture_ex(
			self.world.borrow()
			.hive.borrow_mut()
			.state_as_tex(&self.rd.assets.clone().unwrap()),
			mapp.x, mapp.y, COL_MARKER_FOOD,
			DrawTextureParams {
				dest_size: Some(NEST_DIMS*self.rd.scale_unit(1.)),
				..DrawTextureParams::default()
			});
		self.objs.render(&self.rd);

		self.render_map_vignette(&NEST_TOPLEFT, &NEST_DIMS, 1.9);

		let f = self.objs.objects.iter().find(|(_, o)| match o { Gobj::Fader(_) => true, _ => false });
		if f.is_some() {
			f.unwrap().1.render(&self.rd);
		}
		// TODO remove; debug
		if is_key_down(KeyCode::C) { self.render_debug_map_col() }
    }

    fn unload(&mut self) {}
}

