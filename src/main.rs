use std::{cell::RefCell, rc::Rc};

use hive::Hive;
use macroquad::prelude::*;

mod config;
mod resources;
mod arse;
mod signals;
mod prelude;
mod game_objects;
mod hive;
mod nest;
use nest::Nest;
use prelude::*;
mod gameplay;
use gameplay::*;
mod gobj;
mod ants;
mod markers;
mod food;
mod world;

#[macroquad::main(window_conf())]
async fn main() {
	let assets = Assets::load().await;
	let hive = Rc::new(RefCell::new(Hive::new()));
	let mut ctx = Context::new(
		vec![
			Box::new(Nest::new(hive.clone())),
			Box::new(Gameplay::new(hive.clone())),
		]
	);
	ctx.init(assets);

	let rt = render_target(W as u32, H as u32);
	rt.texture.set_filter(FilterMode::Nearest);
	let mut camera = Camera2D::from_display_rect(Rect { x: 0., y: 0., w: W, h: H });
	camera.render_target = Some(rt);

	loop {
		//if is_key_down(KeyCode::LeftAlt) {
		//	for _ in 0..100 {
		//		ctx.update();
		//	}
		//}

		ctx.update();

		//if !is_key_down(KeyCode::LeftControl) {
			set_camera(&camera);
		//}
		clear_background(DARKGRAY);
		ctx.render();

		//if !is_key_down(KeyCode::LeftControl) {
			set_default_camera();
			draw_texture_ex(rt.texture, 0.0, 0.0, WHITE,
				DrawTextureParams {
					dest_size: Some(vec2(screen_width(), screen_height())),
					flip_y: true,
					..DrawTextureParams::default()
				});
		//}

		//count_and_render_fps();
		
		ctx.handle_signals();
		next_frame().await;
	}
}
