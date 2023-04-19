mod display;
mod draw;
mod mesh;

use draw::DisplayBuffer;

use crate::mesh::get_cube_mesh;

const BUF_WIDTH: usize = 26;
const BUF_HEIGHT: usize = 26;

fn main() {
	let mut display_buf = DisplayBuffer([[0; BUF_WIDTH]; BUF_HEIGHT]);

	let mesh = get_cube_mesh([12.0, 12.0, 0.0].into(), 12.0);

	display_buf.draw_mesh(&mesh);
	println!("{}", display_buf);
}
