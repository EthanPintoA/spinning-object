mod display;
mod draw;
mod mesh;

use draw::DisplayBuffer;

use crate::mesh::Triangle;

const BUF_WIDTH: usize = 26;
const BUF_HEIGHT: usize = 26;

fn main() {
	let mut display_buf = DisplayBuffer([[0; BUF_WIDTH]; BUF_HEIGHT]);

	let mesh = [
		Triangle::new([
			[1.0, 1.0, 0.0].into(),
			[22.0, 4.0, 0.0].into(),
			[4.0, 22.0, 0.0].into(),
		]),
		Triangle::new([
			[22.0, 4.0, 0.0].into(),
			[10.0, 22.0, 0.0].into(),
			[24.0, 20.0, 0.0].into(),
		]),
	];

	display_buf.draw_mesh(&mesh);
	println!("{}", display_buf);
}
