use macroquad::prelude::*;

mod config;
mod resources;
mod arse;
mod signals;
mod prelude;
mod game_objects;
use prelude::*;

const PLAYER_SPEED : f32 = 15.0;
#[derive(PartialEq, Debug)]
enum Gobj {
	Player(Vec2, f32)
}
impl GameObject for Gobj {
	fn update(&mut self) -> bool {
		let d = get_frame_time();
		use Gobj::*;
		match self {
			Player(pos, _r) => {
				let iv = get_ivn();
				*pos += iv*d*PLAYER_SPEED;
				true
			}
		}
	}
	fn render(&self, rd : &RenderData) {
		use Gobj::*;
		let co = rd.camera_offset();
		match self {
			Player(pos, r) => draw_circle(pos.x - co.x, pos.y - co.y, *r, RED),
		}
	}
}

struct Gameplay {
	objs : ObjectSet<Gobj>,
	player_id : GameObjectID,
	rd : RenderData,
}
impl Gameplay {
	fn new() -> Self {
		Gameplay {
			objs: ObjectSet::new(),
			player_id: 0,
			rd: RenderData::new(),
		}
	}
	fn player_pos(&self) -> Vec2 {
		match self.objs.get_obj(self.player_id) {
			Gobj::Player(pos, _) => *pos,
			_ => panic!("player_id ({}) nor pointing to a Player!", self.player_id)
		}
	}
}
impl Scene for Gameplay {
	fn init(&mut self, _a : &Assets) {
		use Gobj::*;
		self.player_id = self.objs.create(Player(vec2(0., 0.), 5.));
	}
    fn update(&mut self, _q : &mut SignalQueue) {
		let d = get_frame_time();
        self.objs.update();
		self.rd.camera_pos = lerp(
			self.rd.camera_pos,
			self.player_pos() + get_ivn()*10.,
			d*6.);
    }

    fn render(&mut self, _q : &mut SignalQueue) {
		let co = self.rd.camera_offset();
		draw_checkerboard_quicker(-co.x, -co.y, 15., DARKGRAY, GRAY);
        self.objs.render(&self.rd);
    }
}

#[macroquad::main(window_conf())]
async fn main() {
	let assets = Assets::load().await;
	let mut ctx = Context::new(
		vec![Box::new(Gameplay::new())]
	);
	ctx.init(assets);

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
