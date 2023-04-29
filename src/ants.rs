use macroquad::prelude::Vec2;

#[derive(PartialEq, Debug)]
pub enum Marker {
	Home(Vec2, f32),
	Food(Vec2, f32)
}
#[derive(PartialEq, Debug)]
pub enum AntState {
	Wander(Vec2),
	Follow(Option<Marker>),
	GoHome(Option<Marker>),
}
