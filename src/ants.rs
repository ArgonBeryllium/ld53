use macroquad::prelude::Vec2;
use crate::{markers::Marker, food::Food};

#[derive(PartialEq, Debug, Clone)]
pub enum AntState {
	Wander(f32, f32, f32),
	GetFood(Vec2),
	Follow(Option<Marker>),
	GoHome(Food, Option<Marker>, f32, f32, f32),
}
