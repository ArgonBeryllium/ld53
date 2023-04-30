use crate::food::Food;
use crate::markers::Marker;
use crate::world::MAP_DIMS;
use crate::world::World;
use std::cell::RefCell;
use std::f32::consts::PI;
use std::rc::Rc;

use macroquad::prelude::*;
use crate::prelude::*;
use crate::ants::*;

const PLAYER_SPEED : f32 = ANT_SPEED*3.;
pub const PLAYER_RAD : f32 = 4.;
const PLAYER_PICKUP_RANGE : f32 = PLAYER_RAD * 2.0;

const ANT_SPEED : f32 = 15.0;
pub const ANT_RAD : f32 = PLAYER_RAD * 0.8;
const ANT_TURN_SPEED : f32 = 8.0;
const ANT_WANDER_TURN_SPEED : f32 = 0.8;
pub const ANT_MARKER_DIST : f32 = ANT_SPEED;
const ANT_FOOD_DETECTION_RANGE : f32 = ANT_RAD * 11.;
const ANT_FOOD_PICKUP_RANGE : f32 = ANT_RAD;
pub const ANT_HOME_DEPOSIT_RANGE : f32 = ANT_RAD*7.;

pub const HOME_MARKER_LIFE : f32 = 60.0;
pub const FOOD_MARKER_LIFE : f32 = 35.0;

pub const HOME_POS : Vec2 = Vec2::ZERO;
pub const HARD_BOUNDS : Vec2 = Vec2::new(MAP_DIMS.x/2., MAP_DIMS.y/2.);

#[derive(Debug, Clone)]
pub enum ParticleStyle {
	Explosive(f32, f32),
	Orbit(f32, f32)
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Gobj {
	Player(Rc<RefCell<Vec<Gobj>>>, Rc<RefCell<World>>, Vec2, Option<Marker>, Vec2, Option<Food>),
	Ant(Rc<RefCell<Vec<Gobj>>>, Rc<RefCell<World>>, Vec2, Vec2, f32, Vec2, AntState),
	Scout(Rc<RefCell<Vec<Gobj>>>, Rc<RefCell<World>>, Vec2, Vec2, f32, Vec2),
	Particles(f32, f32, Color, Vec2, ParticleStyle, Vec<Vec2>, Vec<Vec2>, Vec<f32>),
}
impl Gobj {
	pub fn new_ant(sq : Rc<RefCell<Vec<Gobj>>>, w : Rc<RefCell<World>>, pos : &Vec2) -> Self {
		if rand::gen_range(0., 1.) < 0.1 {
			return Gobj::Scout(sq, w, *pos, *pos, 0., *pos)
		}
		Gobj::Ant(sq, w, *pos, *pos, 0., *pos, AntState::Wander(0., 0., 0.))
	}
	pub fn new_player(sq : Rc<RefCell<Vec<Gobj>>>, w : Rc<RefCell<World>>, pos : &Vec2) -> Self {
		Gobj::Player(sq, w, *pos, None, *pos, None)
	}
	pub fn new_particles(pos : &Vec2, count : usize, life : f32, radius : f32, col : Color, style : ParticleStyle) -> Self {
		let mut positions = Vec::new();
		let mut velocities = Vec::new();
		let mut lives = Vec::new();
		match style {
			ParticleStyle::Explosive(initial, _dissipation) =>
				for _ in 0..count {
					positions.push(random_ring_point(pos, 0., radius));
					velocities.push(random_ring_point(&Vec2::ZERO, initial * 0.8, initial * 1.2));
					lives.push(life * rand::gen_range(0.5, 1.));
				},
			ParticleStyle::Orbit(force, _damp) =>
				for _ in 0..count {
					let p = random_ring_point(pos, 0., radius);
					positions.push(p);
					velocities.push((p-*pos)*force*rand::gen_range(-0.8, 0.8));
					lives.push(life * rand::gen_range(0.5, 1.));
				},
		}
		Gobj::Particles(life, life, col, *pos, style, positions, velocities, lives)
	}
}
impl Gobj {
	fn translate_collidable(pos : &mut Vec2, delta : Vec2, world : Rc<RefCell<World>>) -> bool {
		if world.borrow().is_collision(pos) {
			panic!("Stuck agent at {pos}");
		}
		if !world.borrow().is_collision(&(*pos + delta)) {
			*pos += delta;
			true
		}
		else {
			const DIRS : &[Vec2] = &[
				Vec2::X, Vec2::Y, Vec2::NEG_X, Vec2::NEG_Y
			];
			let proj_delta = DIRS.iter()
				.map(|d| delta*d.dot(delta))
				.filter(|v| !world.borrow().is_collision(&(*pos+*v)))
				.max_by(|v, w|
					v.length().partial_cmp(&w.length()).unwrap());
			if proj_delta.is_some() {
				*pos += proj_delta.unwrap();
				true
			} else { false }
		}
	}
}
impl GameObject for Gobj {
	fn update(&mut self) -> bool {
		let d = get_frame_time();
		use Gobj::*;
		match self {
			Player(spawn_queue, world, pos, marker_type, last_marker_pos, carried_food) => {
				if pos.distance(*last_marker_pos) > ANT_MARKER_DIST {
					match marker_type {
						None => (),
						Some(Marker::Home(..)) => world.borrow_mut().marker
							.create_marker(Marker::Home(*pos, HOME_MARKER_LIFE), spawn_queue.clone()),
						Some(Marker::Food(..)) => world.borrow_mut().marker
							.create_marker(Marker::Food(*pos, FOOD_MARKER_LIFE), spawn_queue.clone()),
					}
					*last_marker_pos = *pos;
				}
				if is_key_pressed(KeyCode::Tab) {
					*marker_type = match marker_type {
						None => Some(Marker::Home(*pos, 0.)),
						Some(Marker::Home(..)) => Some(Marker::Food(*pos, 0.)),
						Some(Marker::Food(..)) => None,
					}
				}
				let iv = get_ivn();
				Self::translate_collidable(pos, iv*d*PLAYER_SPEED, world.clone());

				let closest_food_id = world.borrow().food.find_food(pos, &iv);
				let get_closest_food_pos = ||
					world
					.borrow()
					.food
					.get_food(closest_food_id.unwrap())
					.expect("Closest food no longer exists")
					.pos;
				if closest_food_id.is_some()
					&& carried_food.is_none()
					&& get_closest_food_pos().distance(*pos) < PLAYER_PICKUP_RANGE {
					*carried_food = world.borrow_mut()
						.food
						.take_food(closest_food_id.unwrap());
				}
				else if carried_food.is_some()
					&& pos.distance(HOME_POS) < ANT_HOME_DEPOSIT_RANGE {
					// TODO deposit
					*carried_food = None;
				}
				if carried_food.is_some() {
					let cfp = carried_food.clone().unwrap().pos;
					carried_food.as_mut().unwrap().pos = lerp(
						cfp,
						*pos + get_ivn()*PLAYER_RAD,
						d*20.
					);
				}
				true
			},
			Ant(spawn_queue, world, pos, target, target_change_cooldown, last_marker_pos, state) => {
				let heading = *target - *pos;
				let heading =
					if heading.length() != 0.0 { heading.normalize() }
					else { heading };

				use AntState::*;
				let wander = |time_left_until_next_angle : &mut f32, a : &mut f32, avel : &mut f32, target : &mut Vec2| {
						let rtarget = *target - *pos;
						let angle_target = rtarget.y.atan2(rtarget.x);
						let angle_current = angle_lerp(angle_target, *a, ANT_WANDER_TURN_SPEED*d);

						*target = *pos+vec2(angle_current.cos(), angle_current.sin());
						*target += vec2(avel.cos(), avel.sin())*ANT_TURN_SPEED*d;
						*time_left_until_next_angle -= d;
						if *time_left_until_next_angle < 0. {
							*time_left_until_next_angle = rand::gen_range(1., 3.);
							*avel = rand::gen_range(-PI, PI);
							*a = rand::gen_range(-PI, PI)*2.;
						}
					};

				let mut next_marker = None;
				let closest_marker_food = world.borrow().marker
					.local_markers(
						pos,
						&heading,
						&|m| match *m { Marker::Food(..) => true, _ => false })
					.iter()
					.map(|m| m.clone())
					.filter(|m| m.pos().distance(*pos) > ANT_MARKER_DIST)
					.min_by(|a, b| match a.pos().distance(*pos) < b.pos().distance(*pos) {
							true => std::cmp::Ordering::Less,
							false => std::cmp::Ordering::Greater,
						}
					);
				let closest_marker_home = world.borrow().marker
					.local_markers(
						pos,
						&heading,
						&|m| match *m { Marker::Home(..) => true, _ => false })
					.iter()
					.map(|m| m.clone())
					.filter(|m| m.pos().distance(*pos) > ANT_MARKER_DIST)
					.min_by(|a, b| match a.pos().distance(HOME_POS) < b.pos().distance(HOME_POS) {
							true => std::cmp::Ordering::Less,
							false => std::cmp::Ordering::Greater,
						}
					);

				let closest_food_id = world.borrow().food.find_food(pos, &heading);
				let get_closest_food_pos = ||
					world.borrow().food
					.get_food(closest_food_id.unwrap())
					.expect("Closest food no longer exists")
					.pos;

				let mut next_target = target.clone();
				*state = match state {
					Wander(time_left_until_next_angle, a, avel) => {
						next_marker = Some(Marker::Home(*pos, HOME_MARKER_LIFE));
						wander(time_left_until_next_angle, a, avel, target);

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
						next_target = *food_pos;
						if closest_food_id.is_none() {
							Wander(0., random_angle(), 0.)
						} else {
							next_marker = Some(Marker::Food(*pos, FOOD_MARKER_LIFE));
							if food_pos.distance(*pos) < ANT_FOOD_PICKUP_RANGE {
								let f = world.borrow_mut()
									.food
									.take_food(closest_food_id.unwrap());
								match f {
									Some(f) => GoHome(f, closest_marker_home, 0.,0.,0.),
									None => Wander(0.,random_angle(),0.)
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
							next_marker = Some(Marker::Home(*pos, HOME_MARKER_LIFE));
							match m {
								Some(Marker::Food(p, ..)) => {
									next_target = *p;
									Follow(closest_marker_food)
								},
								_ => Wander(0., random_angle(), 0.)
							}
						}
					},
					GoHome(food, m, time_left_until_next_angle, a, avel) => {
						next_marker = Some(Marker::Food(*pos, FOOD_MARKER_LIFE));
						if pos.distance(HOME_POS) < ANT_HOME_DEPOSIT_RANGE {
							// TODO deposit
							Wander(0., random_angle(), 0.)
						}
						else {
							let nm =
								if closest_marker_home.is_some()
									{ closest_marker_home }
								else { closest_marker_food };
							if m.is_some() { next_target = *m.clone().unwrap().pos(); }
							GoHome(food.clone(), nm, *time_left_until_next_angle, *a, *avel)
						}
					},
				};
				*target_change_cooldown -= d;
				if *target_change_cooldown < 0. {
					*target = next_target;
					*target_change_cooldown = rand::gen_range(0.0, 0.1) + ANT_MARKER_DIST/ANT_SPEED;
				}

				if pos != target {
					if !Self::translate_collidable(pos, (*target - *pos).normalize()*ANT_SPEED*d, world.clone()) {
						*target = *pos-heading;
					}
				}
				if last_marker_pos.distance(*pos) > ANT_MARKER_DIST && next_marker.is_some() {
					world.borrow_mut().marker.create_marker(next_marker.unwrap(), spawn_queue.clone());
					*last_marker_pos = *pos;
				}
				true
			},
			Scout(spawn_queue, world, pos, target, target_change_cooldown, last_marker_pos) => {
				if pos.x >  HARD_BOUNDS.x { pos.x =  HARD_BOUNDS.x; }
				if pos.x < -HARD_BOUNDS.x { pos.x = -HARD_BOUNDS.x; }
				if pos.y >  HARD_BOUNDS.y { pos.y = HARD_BOUNDS.y; }
				if pos.y < -HARD_BOUNDS.y { pos.y = -HARD_BOUNDS.y; }
				*target_change_cooldown -= d;
				if *target_change_cooldown < 0. {
					*target = random_ring_point(pos, ANT_MARKER_DIST*3., ANT_MARKER_DIST*10.);
					*target_change_cooldown = ANT_MARKER_DIST/ANT_SPEED;
				}
				if pos != target {
					Self::translate_collidable(pos, (*target - *pos).normalize()*ANT_SPEED*d, world.clone());
				}
				if last_marker_pos.distance(*pos) > ANT_MARKER_DIST {
					world.borrow_mut().marker.create_marker(Marker::Home(*pos, HOME_MARKER_LIFE), spawn_queue.clone());
					*last_marker_pos = *pos;
				}
				true
			},
			Particles(_o_life, life, _col, pos, style, poss, vels, lives) => {
				match style {
					ParticleStyle::Explosive(_, dissipation) =>
						for i in 0..poss.len() {
							poss[i] += vels[i] * d;
							vels[i] = vels[i] - vels[i] * (*dissipation) * d;
							lives[i] -= d;
						},
					ParticleStyle::Orbit(force, damp) =>
						for i in 0..poss.len() {
							poss[i] += vels[i] * d;
							vels[i] += (poss[i] - *pos) * -*force * d;
							vels[i] = vels[i] - vels[i] * (*damp) * d;
							lives[i] -= d;
						},
				}
				*life -= d;
				*life > 0.0
			},
		}
	}
	fn render(&self, rd : &RenderData) {
		use Gobj::*;
		match self {
			Player(_, _, pos, marker_type, _, carried_food) => {
				let col = match marker_type {
					None => GRAY,
					Some(Marker::Home(..)) => COL_MARKER_HOME,
					Some(Marker::Food(..)) => COL_MARKER_FOOD,
				};
				let pos_ = rd.cast_pos(pos);
				let s = rd.scale_unit(PLAYER_RAD);
				draw_circle(pos_.x, pos_.y, s, col);
				if carried_food.is_some() {
					carried_food.clone().unwrap().render(rd);
				}
			},
			Ant(_sq, _w, pos, _target, _tcc, _lmp, state) => {
				let col = match state {
					AntState::Wander(..) => COL_MARKER_HOME,
					AntState::Follow(..) => LIGHTGRAY,
					AntState::GetFood(..) => DARKGREEN,
					AntState::GoHome(..) => COL_MARKER_FOOD,
				};

				let pos = rd.cast_pos(pos);
				let s = rd.scale_unit(ANT_RAD);
				draw_circle(pos.x, pos.y, s, col);
				//draw_line(pos.x, pos.y, target.x, target.y, 1.0+*tcc, MAGENTA);
			},
			Scout(_ow, _mw, pos, _target, _tcc, _lmp) => {
				let col = COL_MARKER_HOME;
				let pos = rd.cast_pos(pos);
				let s = rd.scale_unit(ANT_RAD);
				draw_circle(pos.x, pos.y, s, col);
				quick_text("S", pos, BLACK);
				//draw_line(pos.x, pos.y, target.x, target.y, 1.0+*tcc, MAGENTA);
			},
			Particles(o_life, _life, col, _pos, _style, poss, _vels, lives) => {
				for i in 0..poss.len() {
					if lives[i] < 0.0 { continue; }
					let dim = rd.scale_unit(lives[i]*PARTICLE_SIZE / o_life);
					let pos = rd.cast_pos(&poss[i]) - dim*0.5;
					draw_texture_ex(
						rd.assets.clone().unwrap().tex_fuzzy_0,
						pos.x, pos.y, *col,
						DrawTextureParams {
							dest_size: Some(vec2(dim, dim)),
							..DrawTextureParams::default()
						});
				}
			}
		}
	}
}

