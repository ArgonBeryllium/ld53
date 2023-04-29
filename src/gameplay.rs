use macroquad::prelude::*;
use crate::prelude::*;
use crate::gobj::*;

pub struct Gameplay {
	pub objs : ObjectSet<Gobj>,
	pub player_id : GameObjectID,
	pub rd : RenderData,
}
impl Gameplay {
	pub fn new() -> Self {
		Gameplay {
			objs: ObjectSet::new(),
			player_id: 0,
			rd: RenderData::new(),
		}
	}
	pub fn player_pos(&self) -> Vec2 {
		match self.objs.get_obj(self.player_id) {
			Gobj::Player(pos) => *pos,
			_ => panic!("player_id ({}) nor pointing to a Player!", self.player_id)
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
		self.rd.camera_pos = lerp(
			self.rd.camera_pos,
			self.player_pos() + get_ivn()*10.,
			d*6.);

		let mp = mouse_pos_scaled_rd(&self.rd);
		if is_mouse_button_pressed(MouseButton::Left) {
			self.objs.create(Gobj::new_ant(&mp));
		}
		else if is_mouse_button_down(MouseButton::Right) {
			self.objs.create(Gobj::Pellet(mp));
		}
    }

    fn render(&mut self, _q : &mut SignalQueue) {
		let co = self.rd.camera_offset();
		draw_checkerboard_quicker(-co.x, -co.y, 15., DARKGRAY, GRAY);
        self.objs.render(&self.rd);
    }
}
