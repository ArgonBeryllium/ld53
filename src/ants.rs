use crate::markers::Marker;
#[derive(PartialEq, Debug)]
pub enum AntState {
	Wander(f32, f32, f32),
	Follow(Option<Marker>),
	GoHome(Option<Marker>),
}
