use macroquad::{prelude::*, audio::{Sound, play_sound_once}};

mod config;
mod resources;
mod arse;
mod signals;
mod prelude;
mod game_objects;
use prelude::*;

enum TestObjs {
	Squinkly(Vec2),
	Particle(Vec2, f32)
}
impl GameObject for TestObjs {
	fn update(&mut self) -> bool {
		use TestObjs::*;
		let d = get_frame_time();
		match self {
			Squinkly(_pos) => true,
			Particle(_pos, life) => {
				*life -= d;
				*life > 0.
			}
		}
	}
	fn render(&self) {
		use TestObjs::*;
		match self {
			Squinkly(pos) => draw_rectangle(pos.x, pos.y, 10., 10., BLUE),
			Particle(pos, life) => draw_rectangle(pos.x, pos.y, *life*5., *life*5., GREEN),
		}
	}
}

struct One {
	pos : Vec2,
	objs : ObjectSet<TestObjs>,
	sound : Option<Sound>,
}
impl One {
	fn new() -> Self {
		One {
			pos: Vec2::ZERO,
			objs: ObjectSet::new(),
			sound : None,
		}
	}
}
impl Scene for One {
	fn init(&mut self, a : &Assets) {
		self.sound = Some(a.sound);
	}
	fn render(&mut self, _q : &mut SignalQueue) {
		let t = get_time() as f32;
		draw_checkerboard(-t * 20., 0., 8., ORANGE, YELLOW);
		let (w, h) = (10., 10.);
		let (x, y) = (self.pos.x - w/2., self.pos.y - h/2.);
		draw_rectangle(x, y, w, h, PURPLE);

		self.objs.render();
		quick_text(&format!("object count: {}", self.objs.objects.len()), self.pos)
	}
	fn update(&mut self, q : &mut SignalQueue) {
		let d = get_frame_time();
		self.pos = lerp(self.pos, mouse_pos_scaled(), d*5.);
		self.objs.update();
		
		if is_mouse_button_pressed(MouseButton::Left) {
			self.objs.objects.push(TestObjs::Particle(mouse_pos_scaled(), 1.))
		}
		if is_mouse_button_pressed(MouseButton::Right) {
			self.objs.objects.push(TestObjs::Squinkly(mouse_pos_scaled()));
			play_sound_once(self.sound.unwrap());
		}
		if is_key_pressed(KeyCode::Space) {
			q.send(Signal::SetScene(1));
		}
	}
}
struct Two;
impl Scene for Two {
    fn update(&mut self, q : &mut SignalQueue) {
		if is_key_pressed(KeyCode::Space) {
			q.send(Signal::SetScene(0));
		}
    }

    fn render(&mut self, _q : &mut SignalQueue) {
		let t = get_time() as f32;
		draw_checkerboard(t*20., t*18., 30., GRAY, DARKGRAY);
    }
}

#[macroquad::main(window_conf())]
async fn main() {
	let assets = Assets::load().await;
	let mut ctx = Context::new(
		assets,
		vec![Box::new(One::new()), Box::new(Two{})]
	);
	ctx.init();

	let rt = render_target(W as u32, H as u32);
	rt.texture.set_filter(FilterMode::Nearest);
	let mut camera = Camera2D::from_display_rect(Rect { x: 0., y: 0., w: W, h: H });
	camera.render_target = Some(rt);
	loop {
		ctx.update();

		set_camera(&camera);
		clear_background(DARKGRAY);
		ctx.render();

		set_default_camera();
		draw_texture_ex(rt.texture, 0.0, 0.0, WHITE,
			DrawTextureParams {
				dest_size: Some(vec2(screen_width(), screen_height())),
				flip_y: true,
				..DrawTextureParams::default()
			});

		count_and_render_fps();
		
		ctx.handle_signals();
		next_frame().await;
	}
}
