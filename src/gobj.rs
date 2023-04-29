use crate::food::FoodWorld;
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
const ANT_FOOD_DETECTION_RANGE : f32 = ANT_SPEED * 3.;
const ANT_FOOD_PICKUP_RANGE : f32 = ANT_RAD * 5.;

pub const HOME_MARKER_LIFE : f32 = 25.0;
pub const FOOD_MARKER_LIFE : f32 = 35.0;

#[derive(Debug)]
pub enum Gobj {
	Player(Vec2),
	Ant(Rc<RefCell<MarkerWorld>>, Rc<RefCell<FoodWorld>>, Vec2, Vec2, Vec2, AntState),
}
impl Gobj {
	pub fn new_ant(mw : Rc<RefCell<MarkerWorld>>, fw : Rc<RefCell<FoodWorld>>, pos : &Vec2) -> Self {
		Gobj::Ant(mw, fw, *pos, *pos, *pos, AntState::Wander(0., 0., 0.))
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
			Ant(marker_world, food_world, pos, target, last_marker_pos, state) => {
				let heading = *target - *pos;
				let heading =
					if heading.length() != 0.0 { heading.normalize() }
					else { heading };
				let closest_marker_food = marker_world.borrow_mut()
					.detect_marker(
						pos,
						&heading,
						&|m| match *m { Marker::Food(..) => true, _ => false }
					);
				let closest_marker_home = marker_world.borrow_mut()
					.detect_marker(
						pos,
						&heading,
						&|m| match *m { Marker::Home(..) => true, _ => false }
					);

				let closest_food_id = food_world.borrow().find_food(pos, &heading);
				let get_closest_food_pos = ||
					food_world
					.borrow()
					.get_food(closest_food_id.unwrap())
					.expect("Closest food no longer exists")
					.pos;

				use AntState::*;

				let mut next_marker = None;
				let next_state = match state {
					Wander(time_left_until_next_angle, a, avel) => {
						let rtarget = *target - *pos;
						let angle_target = rtarget.y.atan2(rtarget.x);
						let angle_current = lerp(angle_target, *a, ANT_WANDER_TURN_SPEED*d);

						*target = *pos+vec2(angle_current.cos(), angle_current.sin());
						*target += vec2(avel.cos(), avel.sin())*ANT_TURN_SPEED*d;
						*time_left_until_next_angle -= d;
						if *time_left_until_next_angle < 0. {
							*time_left_until_next_angle = rand::gen_range(1., 3.);
							*avel = rand::gen_range(-PI, PI);
							*a = rand::gen_range(-PI, PI)*2.;
						}
						next_marker = Some(Marker::Home(*pos, HOME_MARKER_LIFE));

						if closest_food_id.is_some() &&
							get_closest_food_pos().distance(*pos)
								< ANT_FOOD_DETECTION_RANGE
						{
							next_marker = Some(Marker::Food(*pos, FOOD_MARKER_LIFE));
							GetFood(get_closest_food_pos())
						} else if closest_marker_food.is_some() {
							Follow(closest_marker_food)
						}
						else {
							state.clone()
						}
					},
					GetFood(food_pos) => {
						*target = *food_pos;
						if closest_food_id.is_none() {
							Wander(0., 0., 0.)
						} else {
							next_marker = Some(Marker::Food(*pos, FOOD_MARKER_LIFE));
							if food_pos.distance(*pos) < ANT_FOOD_PICKUP_RANGE {
								let f = food_world
									.borrow_mut()
									.take_food(closest_food_id.unwrap());
								match f {
									Some(f) => GoHome(f, closest_marker_home),
									None => Wander(0.,0.,0.)
								}
							} else {
								state.clone()
							}
						}
					},
					Follow(m) => {
						if closest_food_id.is_some() &&
							get_closest_food_pos().distance(*pos)
								< ANT_FOOD_DETECTION_RANGE
						{
							next_marker = Some(Marker::Food(*pos, FOOD_MARKER_LIFE));
							GetFood(get_closest_food_pos())
						} else {
							match m {
								Some(Marker::Food(p, ..)) => {
									*target = *p;
									Follow(closest_marker_food)
								},
								_ => Wander(0., 0., 0.)
							}
						}
					},
					GoHome(_food, _m) => {
						next_marker = Some(Marker::Food(*pos, FOOD_MARKER_LIFE));
						state.clone()
					},
				};
				*state = next_state;

				if pos != target {
					*pos += (*target - *pos).normalize()*ANT_SPEED*d;
				}
				if last_marker_pos.distance(*pos) > ANT_MARKER_DIST && next_marker.is_some() {
					marker_world.borrow_mut().create_marker(next_marker.unwrap());
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
			Ant(_mw, _fw, pos, target, _lmp, state) => {
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

