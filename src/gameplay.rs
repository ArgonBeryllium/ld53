use std::cell::RefCell;
use std::rc::Rc;

use macroquad::prelude::*;
use crate::food::Food;
use crate::food::FoodWorld;
use crate::markers::Marker;
use crate::markers::MarkerWorld;
use crate::prelude::*;
use crate::gobj::*;

pub struct Gameplay {
	pub objs : ObjectSet<Gobj>,
	pub player_id : GameObjectID,
	pub rd : RenderData,
	pub markers : Rc<RefCell<MarkerWorld>>,
	pub food : Rc<RefCell<FoodWorld>>,
	pub spawn_queue : Rc<RefCell<Vec<Gobj>>>,
}
impl Gameplay {
	pub fn new() -> Self {
		Gameplay {
			objs: ObjectSet::new(),
			player_id: 0,
			rd: RenderData::new(),
			markers: Rc::new(RefCell::new(MarkerWorld::new(55.))),
			food: Rc::new(RefCell::new(FoodWorld::new(42.))),
			spawn_queue: Rc::new(RefCell::new(Vec::new())),
		}
	}
	pub fn player_pos(&self) -> Vec2 {
		match self.objs.get_obj(self.player_id) {
			Gobj::Player(_, _, _, pos, _, _) => *pos,
			_ => panic!("player_id ({}) not pointing to a Player!", self.player_id)
		}
	}
}
impl Scene for Gameplay {
	fn init(&mut self, a : &Assets) {
		self.player_id = self.objs.create(
			Gobj::new_player(
				self.spawn_queue.clone(),
				self.markers.clone(),
				self.food.clone(),
				&Vec2::ZERO
				)
			);
		self.rd.init(a);
	}
    fn update(&mut self, _q : &mut SignalQueue) {
		let d = get_frame_time();
        self.objs.update();
		self.markers.borrow_mut().update(d);
		self.food.borrow_mut().update(d);
		if !self.markers.borrow()
			.local_markers(&HOME_POS, &Vec2::ZERO, &|_| true)
				.iter()
				.any(|m| *m.pos() == HOME_POS) {
			self.markers.borrow_mut().create_marker(Marker::Home(HOME_POS, HOME_MARKER_LIFE), self.spawn_queue.clone());
		}
		for obj in self.spawn_queue.borrow().iter() {
			self.objs.create(obj.clone());
		}
		self.spawn_queue.borrow_mut().clear();

		self.rd.camera_pos = lerp(
			self.rd.camera_pos,
			self.player_pos() + get_ivn()*10.,
			d*6.);

		let _mp = mouse_pos_scaled_rd(&self.rd);
		if is_mouse_button_pressed(MouseButton::Left) {
			self.objs.create(Gobj::new_particles(
					&mouse_pos_scaled_rd(&self.rd),
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
					&mouse_pos_scaled_rd(&self.rd),
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
						self.markers.clone(),
						self.food.clone(),
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
				self.food.borrow_mut().put_food(Food::new(
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
		let co = self.rd.camera_offset();

		clear_background(COL_BG);
		const TS : f32 = 40.;
		let mut tp = self.player_pos();
		tp.x %= TS;
		tp.y %= TS;
		let mut x = -tp.x - TS;
		while x < W {
			let mut y = -tp.y - TS;
			while y < H {
				draw_texture_ex(self.rd.assets.clone().unwrap().tex_test, x, y, WHITE,
					DrawTextureParams {
						dest_size: Some(vec2(TS, TS)),
						..DrawTextureParams::default()
					});
				y += TS;
			}
			x += TS;
		}

		draw_circle(HOME_POS.x - co.x, HOME_POS.y - co.y, ANT_HOME_DEPOSIT_RANGE, DARKBLUE);
		draw_rectangle(-HARD_BOUNDS.x - co.x, -HARD_BOUNDS.y - co.y,
			HARD_BOUNDS.x*2., HARD_BOUNDS.y*2., Color::from_rgba(0,55,55,55));

		self.markers.borrow().render(&self.rd);
		self.food.borrow().render(&self.rd);
        self.objs.render(&self.rd);

		quick_text(&format!("objs: {}", self.objs.objects.len()), vec2(mouse_position().0, mouse_position().1), WHITE);
    }
}
