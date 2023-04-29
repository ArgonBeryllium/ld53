use std::process::exit;
use macroquad::prelude::*;
use crate::prelude::*;

pub struct Context {
	signal_queue : SignalQueue,
	pub scene_manager : SceneManager,
}
impl Context {
	pub fn new(scenes : Vec<Box<dyn Scene>>) -> Self {
		Context {
			signal_queue: SignalQueue::new(),
			scene_manager: SceneManager::new(scenes),
		}
	}
	pub fn init(&mut self, assets : Assets) {
		self.scene_manager.init(&assets);
	}

	pub fn update(&mut self) {
		if is_key_pressed(KeyCode::Q) {
			exit(0);
		}
		self.signal_queue.append(self.scene_manager.update());
	}
	pub fn render(&mut self) {
		self.signal_queue.append(self.scene_manager.render());
	}
	pub fn handle_signals(&mut self) {
		loop {
			let s = self.signal_queue.signals.pop_back();
			if s.is_none() { break }
			use Signal::*;
			match s.unwrap() {
				SetScene(i) => self.scene_manager.set_active_scene(i),
			}
		}
	}
}
