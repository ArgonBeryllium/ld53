use macroquad::texture::Texture2D;

use crate::prelude::Assets;



#[derive(Debug)]
pub struct Hive {
	pub life : f32,
	player_gave : bool,
}

pub const HIVE_MAX_LIFE : f32 = 500.;
pub const HIVE_START_LIFE : f32 = HIVE_MAX_LIFE/2.;
impl Hive {
	pub fn new() -> Self {
		Hive {
			life: HIVE_START_LIFE,
			player_gave : false,
		}
	}
	pub fn deposit(&mut self, is_player : bool, value : f32) {
		if is_player {
			self.player_gave = true;
		}
		self.life += value*5.;
		self.life = self.life.max(HIVE_MAX_LIFE);
	}
	pub fn update(&mut self, d : f32) {
		self.life -= d;
	}
	pub fn lost(&self) -> bool { self.life < 0.0 }
	pub fn did_player_give(&mut self) -> bool {
		if self.player_gave {
			self.player_gave = false;
			true
		} else { false }
	}
	pub fn state_as_tex(&self, a : &Assets) -> Texture2D {
		let i = (
				(self.life / HIVE_MAX_LIFE) *
				(a.tex_hive_food.len() as f32)
			).min((a.tex_hive_food.len()-1) as f32) as usize;
		a.tex_hive_food[i]
	}
	pub fn state_as_float(&self) -> f32 { self.life / HIVE_MAX_LIFE }
}
