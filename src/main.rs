mod display;
mod draw;

use draw::DisplayBuffer;
use glam::Vec2;

const BUF_WIDTH: usize = 26;
const BUF_HEIGHT: usize = 26;

fn main() {
	let mut display_buf = DisplayBuffer([[0; BUF_WIDTH]; BUF_HEIGHT]);

	let mesh = [
		[
			Vec2::new(1.0, 1.0),
			Vec2::new(22.0, 4.0),
			Vec2::new(4.0, 22.0),
		],
		[
			Vec2::new(22.0, 4.0),
			Vec2::new(10.0, 22.0),
			Vec2::new(24.0, 20.0),
		],
	];

	display_buf.draw_mesh(&mesh);
	println!("{}", display_buf);
}
