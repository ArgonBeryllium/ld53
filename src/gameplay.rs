use std::cell::RefCell;
use std::rc::Rc;

use macroquad::prelude::*;
use crate::food::Food;
use crate::food::FoodWorld;
use crate::markers::MarkerWorld;
use crate::prelude::*;
use crate::gobj::*;

pub struct Gameplay {
	pub objs : ObjectSet<Gobj>,
	pub player_id : GameObjectID,
	pub rd : RenderData,
	pub markers : Rc<RefCell<MarkerWorld>>,
	pub food : Rc<RefCell<FoodWorld>>,
}
impl Gameplay {
	pub fn new() -> Self {
		Gameplay {
			objs: ObjectSet::new(),
			player_id: 0,
			rd: RenderData::new(),
			markers: Rc::new(RefCell::new(MarkerWorld::new(55.))),
			food: Rc::new(RefCell::new(FoodWorld::new(42.))),
		}
	}
	pub fn player_pos(&self) -> Vec2 {
		match self.objs.get_obj(self.player_id) {
			Gobj::Player(pos) => *pos,
			_ => panic!("player_id ({}) not pointing to a Player!", self.player_id)
		}
	}
}
impl Scene for Gameplay {
	fn init(&mut self, _a : &Assets) {
		use Gobj::*;
		self.player_id = self.objs.create(Player(vec2(0., 0.)));
	}
    fn update(&mut self, _q : &mut SignalQueue) {
		let d = get_frame_time();
        self.objs.update();
		self.markers.borrow_mut().update(d);
		self.food.borrow_mut().update(d);

		self.rd.camera_pos = lerp(
			self.rd.camera_pos,
			self.player_pos() + get_ivn()*10.,
			d*6.);

		let mp = mouse_pos_scaled_rd(&self.rd);
		if is_mouse_button_pressed(MouseButton::Left) {
			self.objs.create(Gobj::new_ant(self.markers.clone(), self.food.clone(), &mp));
		}
		else if is_mouse_button_pressed(MouseButton::Right) {
			self.food.borrow_mut().put_food(Food::new(&mp));
		}
    }

    fn render(&mut self, _q : &mut SignalQueue) {
		let co = self.rd.camera_offset();

		draw_checkerboard_quicker(-co.x, -co.y, 15., DARKGRAY, GRAY);
		draw_circle(HOME_POS.x - co.x, HOME_POS.y - co.y, ANT_HOME_DEPOSIT_RANGE, DARKBLUE);
		draw_rectangle(-HARD_BOUNDS.x - co.x, -HARD_BOUNDS.y - co.y,
			HARD_BOUNDS.x*2., HARD_BOUNDS.y*2., Color::from_rgba(0,55,55,55));

        self.objs.render(&self.rd);

		self.markers.borrow().render(&self.rd);
		self.food.borrow().render(&self.rd);
    }
}
