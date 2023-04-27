pub trait GameObject {
	fn update(&mut self) -> bool { true }
	fn render(&self);
}
pub struct ObjectSet<T : GameObject> {
	pub objects : Vec<T>
}
impl<T : GameObject> ObjectSet<T> {
	pub fn new() -> Self {
		ObjectSet { objects: Vec::new() }
	}
	pub fn update(&mut self) {
		let mut i = 0;
		while i < self.objects.len() {
			if !self.objects[i].update() {
				self.objects.remove(i);
			} else { i += 1 }
		}
	}
	pub fn render(&mut self) {
		for o in self.objects.iter() {
			o.render();
		}
	}
}

