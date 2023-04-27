use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub enum Signal {
	SetScene(usize)
}

#[derive(Clone, Debug)]
pub struct SignalQueue {
	pub signals : VecDeque<Signal>,
}
impl SignalQueue {
	pub fn new() -> Self { SignalQueue { signals: VecDeque::new() } }
	pub fn send(&mut self, sig : Signal) {
		self.signals.push_front(sig);
	}
	pub fn append(&mut self, mut other : SignalQueue) {
		self.signals.append(&mut other.signals);
	}
}
