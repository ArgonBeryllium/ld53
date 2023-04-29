use crate::markers::Marker;
use crate::markers::MarkerWorld;
use std::cell::RefCell;
use std::f32::consts::PI;
use std::rc::Rc;

use macroquad::prelude::*;
use crate::prelude::*;
use crate::ants::*;

const PLAYER_SPEED : f32 = ANT_SPEED*3.;
const PLAYER_RAD : f32 = 4.;

const ANT_SPEED : f32 = 15.0;
const ANT_RAD : f32 = PLAYER_RAD * 0.8;
const ANT_TURN_SPEED : f32 = 8.0;
const ANT_WANDER_TURN_SPEED : f32 = 0.8;
const ANT_MARKER_DIST : f32 = ANT_SPEED;

pub const HOME_MARKER_LIFE : f32 = 3.0;
pub const FOOD_MARKER_LIFE : f32 = 3.0;

#[derive(PartialEq, Debug)]
pub enum Gobj {
	Player(Vec2),
	Ant(Rc<RefCell<MarkerWorld>>, Vec2, Vec2, Vec2, AntState),
}
impl Gobj {
	pub fn new_ant(mw : Rc<RefCell<MarkerWorld>>, pos : &Vec2) -> Self {
		Gobj::Ant(mw, *pos, *pos, *pos, AntState::Wander(0., 0., 0.))
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
			Ant(marker_world, pos, target, last_marker_pos, state) => {
				let heading = *target - *pos;
				let heading =
					if heading.length() != 0.0 { heading.normalize() }
					else { heading };
				let closest_marker = marker_world.borrow_mut().detect_marker(pos, &heading);
				use AntState::*;

				let mut spawn_marker = None;
				match state {
					Wander(left, a, avel) => {
						let rtarget = *target - *pos;
						let angle_target = rtarget.y.atan2(rtarget.x);
						let angle_current = lerp(angle_target, *a, ANT_WANDER_TURN_SPEED*d);

						*target = *pos+vec2(angle_current.cos(), angle_current.sin());
						*target += vec2(avel.cos(), avel.sin())*ANT_TURN_SPEED*d;
						*left -= d;
						if *left < 0. {
							*left = rand::gen_range(1., 3.);
							*avel = rand::gen_range(-PI, PI);
							*a = rand::gen_range(-PI, PI)*2.;
						}

						if pos != target {
							*pos += (*target - *pos).normalize()*ANT_SPEED*d;
						}

						spawn_marker = Some(Marker::Home(*pos, HOME_MARKER_LIFE));
					},
					Follow(m) => {
					},
					GoHome(m) => {
						spawn_marker = Some(Marker::Food(*pos, FOOD_MARKER_LIFE));
					},
				}
				if last_marker_pos.distance(*pos) > ANT_MARKER_DIST && spawn_marker.is_some() {
					marker_world.borrow_mut().create_marker(spawn_marker.unwrap());
					*last_marker_pos = *pos;
				}
				true
			},
		}
	}
	fn render(&self, rd : &RenderData) {
		use Gobj::*;
		let co = rd.camera_offset();
		match self {
			Player(pos) => draw_circle(pos.x - co.x, pos.y - co.y, PLAYER_RAD, RED),
			Ant(_mw, pos, target, _lmp, state) => {
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
		}
	}
}

