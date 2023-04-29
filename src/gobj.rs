use macroquad::prelude::*;
use crate::prelude::*;
use crate::ants::*;

const PLAYER_SPEED : f32 = ANT_SPEED*3.;
const PLAYER_RAD : f32 = 4.;

const ANT_SPEED : f32 = 15.0;
const ANT_RAD : f32 = PLAYER_RAD * 0.8;
const ANT_TURN_SPEED : f32 = 8.0;
const ANT_TARGET_RADIUS : f32 = ANT_RAD * 2.0;

const ANT_MIN_WANDER_DIST : f32 = ANT_RAD * 8.0;
const ANT_MAX_WANDER_DIST : f32 = ANT_RAD * 32.0;

#[derive(PartialEq, Debug)]
pub enum Gobj {
	Player(Vec2),
	Ant(Vec2, Vec2, AntState),
	Pellet(Vec2)
}
impl Gobj {
	pub fn new_ant(pos : &Vec2) -> Self {
		Gobj::Ant(*pos, *pos, AntState::Wander(*pos))
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
			Ant(pos, target, state) => {
				use AntState::*;
				match state {
					Wander(next_target) => {
						*target = lerp(*target, *next_target, d*ANT_TURN_SPEED);
						if (*pos - *next_target).length() < ANT_TARGET_RADIUS {
							*next_target = random_ring_point(
								pos,
								ANT_MIN_WANDER_DIST,
								ANT_MAX_WANDER_DIST
							);
						}
						if pos != target {
							*pos += (*target - *pos).normalize()*ANT_SPEED*d;
						}
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
			Ant(pos, target, state) => {
				let pos = *pos - co;
				let target = *target - co;

				draw_circle(
					pos.x,
					pos.y,
					ANT_RAD*0.9,
					BLUE);
				draw_line(pos.x, pos.y, target.x, target.y, 2., MAGENTA);
				quick_text(&format!("{:?}", state), pos, WHITE);
			},
			Pellet(pos) => {
				draw_circle(pos.x-co.x, pos.y-co.y, PLAYER_RAD*0.5, GREEN);
			}
		}
	}
}

