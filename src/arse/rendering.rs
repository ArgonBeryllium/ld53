use macroquad::prelude::*;
use crate::{config::*, prelude::load_font};

#[rustfmt::skip]
pub fn draw_checkerboard(x: f32, y: f32, s: f32, c1: Color, c2: Color) {
	let x = if x < 0. { -(-x % (2. * s)) } else { x % (2. * s) } - (3. * s);
	let y = if y < 0. { -(-y % (2. * s)) } else { y % (2. * s) } - (3. * s);
	let mut x_ = 0;
	let mut px = x;
	while px < W {
		let mut py = y;
		let mut y_ = 0;
		while py < H {
			let i = x_ + y_;
			draw_rectangle(px, py, s, s, if (i % 2) == 0 { c1 } else { c2 });
			py += s;
			y_ += 1;
		}
		px += s;
		x_ += 1;
	}
}
#[rustfmt::skip]
pub fn draw_checkerboard_quicker(x: f32, y: f32, s: f32, c1: Color, c2: Color) {
	draw_rectangle(0., 0., W, H, c1);
	let x = if x < 0. { -(-x % (2. * s)) } else { x % (2. * s) } - (3. * s);
	let y = if y < 0. { -(-y % (2. * s)) } else { y % (2. * s) } - (3. * s);
	let mut x_ = 0;
	let mut px = x;
	while px < W {
		let mut py = y;
		let mut y_ = 0;
		while py < H {
			let i = x_ + y_;
			if (i % 2) != 0 {
				draw_rectangle(px, py, s, s,  c2);
			}
			py += s;
			y_ += 1;
		}
		px += s;
		x_ += 1;
	}
}

#[allow(non_upper_case_globals)]
pub fn count_and_render_fps() {
	static mut frame_count : u32 = 0;
	static mut display_frame_count : u32 = 0;
	static mut last_frame_time : u32 = 0;
	unsafe {
		frame_count += 1;
		if get_time().floor() as u32 > last_frame_time {
			last_frame_time = get_time().floor() as u32;
			display_frame_count = frame_count;
			frame_count = 0;
		}
		draw_text(&display_frame_count.to_string(), 10.0, 30.0, 40., BLACK);
		draw_text(&display_frame_count.to_string(), 12.0, 32.0, 40., WHITE);
	}
}

pub fn quick_text(text : &str, pos : Vec2, color : Color) {
	static mut FONT: Option<Font> = None;
	unsafe {
		if FONT.is_none() {
			FONT = Some(load_font());
		}
	}
	draw_text_ex(
		text,
		pos.x, pos.y,
		TextParams{
			font_size: DEFAULT_FONT_SIZE as u16,
			font_scale: 1.,
			color,
			font: unsafe { FONT.expect("quick_text font is none") },
			..TextParams::default()
		});
}
