use std::{cell::RefCell, rc::Rc};

use gobj::{Gobj, PLAYER_RAD};
use hive::Hive;
use macroquad::prelude::*;

mod config;
mod resources;
mod arse;
mod signals;
mod prelude;
mod game_objects;
mod hive;
use prelude::*;
mod gameplay;
use gameplay::*;
use world::World;
mod gobj;
mod ants;
mod markers;
mod food;
mod world;

struct Nest {
	rd : RenderData,
	objs : ObjectSet<Gobj>,
	sq : Rc<RefCell<Vec<Gobj>>>,
	world : Rc<RefCell<World>>,
}
impl Nest {
	fn new(hive : Rc<RefCell<Hive>>) -> Self {
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
		draw_texture_ex(self.rd.assets.clone().unwrap().tex_vig, map_vignette_pos.x, map_vignette_pos.y, WHITE,
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

		// TODO remove; debug
		if is_key_down(KeyCode::C) { self.render_debug_map_col() }
    }
}

#[macroquad::main(window_conf())]
async fn main() {
	let assets = Assets::load().await;
	let hive = Rc::new(RefCell::new(Hive::new()));
	let mut ctx = Context::new(
		vec![
			Box::new(Nest::new(hive.clone())),
			Box::new(Gameplay::new(hive.clone())),
		]
	);
	ctx.init(assets);

	let rt = render_target(W as u32, H as u32);
	rt.texture.set_filter(FilterMode::Nearest);
	let mut camera = Camera2D::from_display_rect(Rect { x: 0., y: 0., w: W, h: H });
	camera.render_target = Some(rt);

	loop {
		// TODO remove; debug
		if is_key_down(KeyCode::LeftAlt) {
			for _ in 0..100 {
				ctx.update();
			}
		}

		ctx.update();

		// TODO remove; debug
		if !is_key_down(KeyCode::LeftControl) {
			set_camera(&camera);
		}
		clear_background(DARKGRAY);
		ctx.render();

		if !is_key_down(KeyCode::LeftControl) {
			set_default_camera();
			draw_texture_ex(rt.texture, 0.0, 0.0, WHITE,
				DrawTextureParams {
					dest_size: Some(vec2(screen_width(), screen_height())),
					flip_y: true,
					..DrawTextureParams::default()
				});
		}

		count_and_render_fps();
		
		ctx.handle_signals();
		next_frame().await;
	}
}
