use crate::SignalQueue;
use crate::Assets;

#[allow(unused_variables)]
pub trait Scene {
	fn load(&mut self) {}
	fn unload(&mut self) {}
	fn init(&mut self, a : &Assets) {}
	fn update(&mut self, q : &mut SignalQueue);
	fn render(&mut self, q : &mut SignalQueue);
}
pub struct SceneManager {
	pub scenes : Vec<Box<dyn Scene>>,
	active_index : usize,
}
#[allow(dead_code)]
impl SceneManager {
	pub fn new(scenes : Vec<Box<dyn Scene>>) -> Self {
		SceneManager {
			scenes,
			active_index: 0,
		}
	}
	pub fn get_active_scene(&self) -> &Box<dyn Scene> {
		self.scenes.get(self.get_active_index()).unwrap()
	}
	pub fn get_active_scene_mut(&mut self) -> &mut Box<dyn Scene> {
		self.scenes.get_mut(self.active_index).unwrap()
	}
	pub fn get_active_index(&self) -> usize { self.active_index }
	pub fn set_active_scene(&mut self, i : usize) {
		self.scenes[self.active_index].unload();
		self.active_index = i;
		self.scenes[self.active_index].load();
	}

	pub fn init(&mut self, assets : &Assets) {
		for s in self.scenes.iter_mut() {
			s.init(assets)
		}
		self.scenes[self.active_index].load();
	}
	pub fn update(&mut self) -> SignalQueue {
		let mut q = SignalQueue::new();
		self.get_active_scene_mut().update(&mut q);
		q
	}
	pub fn render(&mut self) -> SignalQueue {
		let mut q = SignalQueue::new();
		self.get_active_scene_mut().render(&mut q);
		q
	}
}
