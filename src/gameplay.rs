use std::cell::RefCell;
use std::rc::Rc;

use macroquad::prelude::*;
use crate::food::Food;
use crate::hive::Hive;
use crate::markers::Marker;
use crate::prelude::*;
use crate::gobj::*;
use crate::world::MAP_DIMS;
use crate::world::MAP_TOPLEFT;
use crate::world::World;

pub const PREVIEW_LENGTH : f32 = 5.0;
pub const FOOD_TIMER_LOW : f32 = 15.0;
pub const FOOD_TIMER_HIGH : f32 = 36.0;
pub enum GameState {
	On,
	Preview(f32),
	Over,
}
pub struct Gameplay {
	pub objs : ObjectSet<Gobj>,
	pub player_id : GameObjectID,
	pub rd : RenderData,
	pub world : Rc<RefCell<World>>,
	pub spawn_queue : Rc<RefCell<Vec<Gobj>>>,
	pub state : GameState,
	load_timer : f32,
	food_timer: f32,
}
impl Gameplay {
	pub fn new(hive : Rc<RefCell<Hive>>) -> Self {
		Gameplay {
			objs: ObjectSet::new(),
			player_id: 0,
			rd: RenderData::new(),
			world : Rc::new(RefCell::new(World::new(hive))),
			spawn_queue: Rc::new(RefCell::new(Vec::new())),
			state: GameState::On,
			load_timer: 1.,
			food_timer: 0.,
		}
	}

	pub fn presim(&mut self) {
		let mut q = SignalQueue::new();

		for _ in 0..100 {
			self.objs.create(Gobj::new_ant(
					self.spawn_queue.clone(),
					self.world.clone(),
					&random_ring_point(
						&HOME_POS,
						ANT_RAD,
						ANT_RAD*3.
					)
				));
		}
		for _ in 0..200 {
			self.update(&mut q);
		}
		match self.state {
			GameState::Over => self.restart(),
			_ => ()
		}
	}
	pub fn spawn_food(&mut self) {
		for _ in 0..3 {
			let mut place;
			loop {
				place = random_ring_point(&HOME_POS, MAP_DIMS.x*0.1, MAP_DIMS.x/2.);
				if !self.world.borrow().is_collision(&place)
					&& place.distance(self.player_pos()) > W {
						break;
				}
			}
			for _ in 0..rand::gen_range(5, 25) {
				self.world.borrow_mut().food.put_food(Food::new(
						&random_ring_point(
							&place,
							ANT_RAD,
							ANT_RAD*6.
							)
						)
					);
			}
		}
	}
	pub fn lose(&mut self) {
		self.state = GameState::Over;
	}
	pub fn restart(&mut self) {
		todo!();
		self.state = GameState::On;
	}
	pub fn player_pos(&self) -> Vec2 {
		match self.objs.get_obj(self.player_id) {
			Gobj::Player(_, _, pos, _, _, _) => *pos,
			_ => panic!("player_id ({}) not pointing to a Player!", self.player_id)
		}
	}
	fn set_player_pos(&mut self, p : &Vec2) {
		match self.objs.get_obj_mut(self.player_id) {
			Gobj::Player(_, _, pos, _, _, _) => *pos = *p,
			_ => panic!("player_id ({}) not pointing to a Player!", self.player_id)
		}
	}
	fn render_bg_tex(&self) {
		const TS : f32 = 128.;
		const TL : u8 = 8;
		let mut tp = self.player_pos();
		tp.x %= TS;
		tp.y %= TS;
		let mut x = -tp.x - TS;
		while x < W {
			let mut y = -tp.y - TS;
			while y < H {
				draw_texture_ex(self.rd.assets.clone().unwrap().tex_sand, x, y, Color::from_rgba(TL,TL,TL,255),
					DrawTextureParams {
						dest_size: Some(vec2(TS, TS)),
						..DrawTextureParams::default()
					});
				y += TS;
			}
			x += TS;
		}
	}
	fn render_map_tex(&self) {
		let mapp = self.rd.cast_pos(&MAP_TOPLEFT);
		draw_texture_ex(self.rd.assets.clone().unwrap().tex_map, mapp.x, mapp.y, WHITE,
			DrawTextureParams {
				dest_size: Some(MAP_DIMS*self.rd.scale_unit(1.)),
				..DrawTextureParams::default()
			});
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
}
impl Scene for Gameplay {
	fn init(&mut self, a : &Assets) {
		self.player_id = self.objs.create(
			Gobj::new_player(
				self.spawn_queue.clone(),
				self.world.clone(),
				&Vec2::ZERO
				)
			);
		self.rd.init(a);
		self.world.borrow_mut().init(&a.tex_map, &MAP_DIMS);

		self.presim();
	}
	fn load(&mut self) {
		self.load_timer = 1.;
		self.objs.create(Gobj::Fader(1.0));
	}
    fn update(&mut self, q : &mut SignalQueue) {
		use GameState::*;
		let d = get_frame_time().min(0.2).max(0.001);
		self.rd.d = d;
		match self.state {
			On => {
				self.world.borrow_mut().hive.borrow_mut().update(d);
				self.objs.update();
				self.world.borrow_mut().marker.update(d);
				self.world.borrow_mut().food.update(d);

				// TODO remove debug condition
				if self.world.borrow().hive.borrow().lost()
					|| is_key_down(KeyCode::L) {
					self.lose();
				}

				if self.food_timer >= 0. {
					self.food_timer -= d;
				} else if self.world.borrow().hive.borrow().state_as_float() < 0.9 {
					self.spawn_food();
					self.food_timer = rand::gen_range(FOOD_TIMER_LOW, FOOD_TIMER_HIGH);
				}

				if self.world.borrow().hive.borrow_mut().did_player_give() {
					self.state = Preview(PREVIEW_LENGTH);
				}
				if self.load_timer >= 0.0 {
					self.load_timer -= d;
				}
				if self.player_pos().distance(HOME_POS) < PLAYER_PICKUP_RANGE
					&& self.load_timer <= 0.0 {
					q.send(Signal::SetScene(0));
					self.set_player_pos(&(HOME_POS+vec2(0.0, ANT_HOME_DEPOSIT_RANGE)));
				}

				for obj in self.spawn_queue.borrow().iter() {
					self.objs.create(obj.clone());
				}
				self.spawn_queue.borrow_mut().clear();
				if !self.world.borrow().marker
					.local_markers(&HOME_POS, &Vec2::ZERO, &|_| true)
						.iter()
						.any(|m| *m.pos() == HOME_POS) {
					self.world.borrow_mut().marker.create_marker(Marker::Home(HOME_POS, HOME_MARKER_LIFE), self.spawn_queue.clone());
				}

				self.rd.camera_pos = lerp(
					self.rd.camera_pos,
					self.player_pos() + get_ivn()*10.,
					d*6.);
				self.rd.zoom = 1.0;
			},
			Preview(_) => {
				self.rd.camera_pos = lerp(
					self.rd.camera_pos,
					self.player_pos(),
					d*6.);
				if let Preview(ref mut left) = self.state {
					*left -= d;
					if *left < 0.0 { self.state = On; }
				}
			},
			Over => {
				if is_key_pressed(KeyCode::R) {
					self.restart();
				}
			}
		}

		// TODO remove; debug
		self.debug_update();
    }

    fn render(&mut self, _q : &mut SignalQueue) {
		use GameState::*;
		match self.state {
			On => {
				self.render_bg_tex();
				self.render_map_tex();
				self.world.borrow().food.render(&self.rd);
				self.objs.render(&self.rd);
				self.render_map_vignette(&MAP_TOPLEFT, &MAP_DIMS, 1.8);
			},
			Preview(left) => {
				clear_background(COL_BG);
				self.render_map_tex();
				//let a = 1.0 - left/PREVIEW_LENGTH;
				//draw_rectangle(0.,0.,W,H,Color{r: COL_BG.r, g: COL_BG.g, b: COL_BG.b, a});

				self.objs.render(&self.rd);
				self.world.borrow().marker.render(&self.rd);
				self.render_map_vignette(&MAP_TOPLEFT, &MAP_DIMS, 1.8);

				self.debug_render();
				self.rd.zoom = lerp(self.rd.zoom, 0.1, self.rd.d);
			},
			Over => {
				clear_background(BLACK);
				self.render_map_tex();
				self.world.borrow().marker.render(&self.rd);
				self.world.borrow().food.render(&self.rd);
				self.objs.render(&self.rd);
				self.render_map_vignette(&MAP_TOPLEFT, &MAP_DIMS, 1.8);

				const VS : f32 = 2.5;
				draw_texture_ex(self.rd.assets.clone().unwrap().tex_vig, W/2.-VS*W/2., H/2.-VS*W/2., RED,
					DrawTextureParams{ dest_size: Some(vec2(W, W)*VS), ..DrawTextureParams::default() });

				self.rd.zoom = lerp(self.rd.zoom, 0.0, self.rd.d*0.4);
			}
		}
		let f = self.objs.objects.iter().find(|(_, o)| match o { Gobj::Fader(_) => true, _ => false });
		if f.is_some() {
			f.unwrap().1.render(&self.rd);
		}

		// TODO remove; debug
		self.debug_render();
		quick_text(&format!("objs: {}", self.objs.objects.len()), vec2(mouse_position().0, mouse_position().1), WHITE);
    }
}
#[allow(dead_code)]
impl Gameplay {
	fn debug_update(&mut self) {
		if is_key_down(KeyCode::LeftShift) {
			self.rd.zoom = 0.5;
		}
		let mp = mouse_pos_scaled_rd(&self.rd);
		if is_mouse_button_pressed(MouseButton::Left) {
			self.objs.create(Gobj::new_particles(
					&mp,
					100,
					3.,
					1.,
					GREEN,
					ParticleStyle::Explosive(100.,
					0.07))
				);
		}
		else if is_mouse_button_pressed(MouseButton::Right) {
			self.objs.create(Gobj::new_particles(
					&mp,
					100,
					3.,
					10.,
					ORANGE,
					ParticleStyle::Orbit(4., 0.03))
				);
		}
		if is_key_pressed(KeyCode::Key1) {
			for _ in 0..20 {
				self.objs.create(Gobj::new_ant(
						self.spawn_queue.clone(),
						self.world.clone(),
						&random_ring_point(
							&mouse_pos_scaled_rd(&self.rd),
							ANT_RAD,
							ANT_RAD*3.
						)
					));
			}
		}
		if is_key_pressed(KeyCode::Key2) {
			self.spawn_food();
		}
	}
	fn debug_render(&mut self) {
		if is_key_down(KeyCode::C) { self.render_debug_map_col() }
		let hcp = self.rd.cast_pos(&HOME_POS);
		draw_circle(hcp.x, hcp.y, self.rd.scale_unit(ANT_HOME_DEPOSIT_RANGE), DARKBLUE);
		let hbp = self.rd.cast_pos(&vec2(-HARD_BOUNDS.x, -HARD_BOUNDS.y));
		let hbs = HARD_BOUNDS*self.rd.scale_unit(1.);
		draw_rectangle_lines(hbp.x, hbp.y, hbs.x*2., hbs.y*2., 1.0, YELLOW);

		let mp = mouse_pos_scaled_rd(&self.rd);
		draw_circle(mp.x, mp.y, 4., PINK);
	}
	fn render_debug_map_col(&self) {
		let css = self.world.borrow().collision_cell_size*self.rd.scale_unit(1.);
		for (p, b) in self.world.borrow().collision_map.iter() {
			let p = self.rd.cast_pos(
				&(vec2(p.0 as f32, p.1 as f32)
				* css
				+ MAP_TOPLEFT));
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
}
