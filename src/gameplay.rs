use std::cell::RefCell;
use std::rc::Rc;

use macroquad::prelude::*;
use crate::food::Food;
use crate::markers::Marker;
use crate::prelude::*;
use crate::gobj::*;
use crate::world::MAP_DIMS;
use crate::world::MAP_TOPLEFT;
use crate::world::World;

pub struct Gameplay {
	pub objs : ObjectSet<Gobj>,
	pub player_id : GameObjectID,
	pub rd : RenderData,
	pub world : Rc<RefCell<World>>,
	pub spawn_queue : Rc<RefCell<Vec<Gobj>>>,
}
impl Gameplay {
	pub fn new() -> Self {
		Gameplay {
			objs: ObjectSet::new(),
			player_id: 0,
			rd: RenderData::new(),
			world : Rc::new(RefCell::new(World::new())),
			spawn_queue: Rc::new(RefCell::new(Vec::new())),
		}
	}
	pub fn player_pos(&self) -> Vec2 {
		match self.objs.get_obj(self.player_id) {
			Gobj::Player(_, _, pos, _, _, _) => *pos,
			_ => panic!("player_id ({}) not pointing to a Player!", self.player_id)
		}
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
		self.world.borrow_mut().init(a);
	}
    fn update(&mut self, _q : &mut SignalQueue) {
		let d = get_frame_time().min(0.2).max(0.001);
		self.rd.d = d;

        self.objs.update();
		self.world.borrow_mut().marker.update(d);
		self.world.borrow_mut().food.update(d);
		if !self.world.borrow().marker
			.local_markers(&HOME_POS, &Vec2::ZERO, &|_| true)
				.iter()
				.any(|m| *m.pos() == HOME_POS) {
			self.world.borrow_mut().marker.create_marker(Marker::Home(HOME_POS, HOME_MARKER_LIFE), self.spawn_queue.clone());
		}
		for obj in self.spawn_queue.borrow().iter() {
			self.objs.create(obj.clone());
		}
		self.spawn_queue.borrow_mut().clear();

		self.rd.camera_pos = lerp(
			self.rd.camera_pos,
			self.player_pos() + get_ivn()*10.,
			d*6.);

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
			for _ in 0..20 {
				self.world.borrow_mut().food.put_food(Food::new(
						&random_ring_point(
							&mouse_pos_scaled_rd(&self.rd),
							ANT_RAD,
							ANT_RAD*3.
							)
						)
					);
			}
		}
    }

    fn render(&mut self, _q : &mut SignalQueue) {
		clear_background(COL_BG);
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

		let mapp = self.rd.cast_pos(&MAP_TOPLEFT);
		draw_texture_ex(self.rd.assets.clone().unwrap().tex_map, mapp.x, mapp.y, WHITE,
			DrawTextureParams {
				dest_size: Some(MAP_DIMS*self.rd.scale_unit(1.)),
				..DrawTextureParams::default()
			});

		self.world.borrow().marker.render(&self.rd);
		self.world.borrow().food.render(&self.rd);
        self.objs.render(&self.rd);

		let map_vignette_scale = 2.;
		let map_vignette_dims = MAP_DIMS*self.rd.scale_unit(map_vignette_scale);
		let map_vignette_pos = self.rd.cast_pos(&(
				MAP_TOPLEFT + MAP_DIMS/2.
				- MAP_DIMS*map_vignette_scale/2.
				)
			);
		draw_texture_ex(self.rd.assets.clone().unwrap().tex_vig, map_vignette_pos.x, map_vignette_pos.y, WHITE,
			DrawTextureParams {
				dest_size: Some(map_vignette_dims),
				..DrawTextureParams::default()
			});

		// TODO remove; debug
		self.debug_render();
		quick_text(&format!("objs: {}", self.objs.objects.len()), vec2(mouse_position().0, mouse_position().1), WHITE);
    }
}
#[allow(dead_code)]
impl Gameplay {
	fn debug_render(&mut self) {
		self.rd.zoom = lerp(self.rd.zoom, if is_key_down(KeyCode::LeftShift) { 0.10 } else { 1.0 }, get_frame_time()*5.);

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
