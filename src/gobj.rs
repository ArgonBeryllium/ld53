use macroquad::prelude::*;
use crate::prelude::*;
use crate::ants::*;

const PLAYER_SPEED : f32 = 15.0;
const PLAYER_RAD : f32 = 4.;
#[derive(PartialEq, Debug)]
pub enum Gobj {
	Player(Vec2),
	Ant(Vec2, AntState),
	Pellet(Vec2)
}
impl Gobj {
	pub fn new_ant(pos : &Vec2) -> Self {
		Gobj::Ant(*pos, AntState::Wander)
	}
}
impl GameObject for Gobj {
	fn update(&mut self) -> bool {
		let d = get_frame_time();
		use Gobj::*;
		match self {
			Player(pos) => {
				let iv = get_ivn();
				*pos += iv*d*PLAYER_SPEED;
				true
			},
			Ant(pos, state) => {
				use AntState::*;
				match state {
					Wander => {

					},
					Follow(m) => {

					},
					GoHome(m) => {

					},
				}
				true
			},
			Pellet(_pos) => true,
		}
	}
	fn render(&self, rd : &RenderData) {
		use Gobj::*;
		let co = rd.camera_offset();
		match self {
			Player(pos) => draw_circle(pos.x - co.x, pos.y - co.y, PLAYER_RAD, RED),
			Ant(pos, state) => {
				draw_circle(
					pos.x - co.x,
					pos.y - co.y,
					PLAYER_RAD*0.9,
					BLUE);
				quick_text(&format!("{:?}", state), *pos - co, WHITE);
			},
			Pellet(pos) => {
				draw_circle(pos.x-co.x, pos.y-co.y, PLAYER_RAD*0.5, GREEN);
			}
		}
	}
}

