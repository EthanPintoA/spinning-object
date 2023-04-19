mod display;
mod draw;
mod mesh;

use std::f32::consts::PI;

use draw::DisplayBuffer;
use glam::{EulerRot, Quat, Vec3};

use crate::mesh::{get_cube_mesh, rotate_mesh};

const BUF_WIDTH: usize = 50;
const BUF_HEIGHT: usize = 50;

const LIGHT_DIRECTION: Vec3 = Vec3::new(0.0, 0.0, 1.0);

fn main() {
	let mut display_buf = DisplayBuffer([[0; BUF_WIDTH]; BUF_HEIGHT]);

	let mesh_center = Vec3::new(BUF_WIDTH as f32 / 2.0, BUF_HEIGHT as f32 / 2.0, 0.0);
	let mut mesh = get_cube_mesh(mesh_center, BUF_HEIGHT as f32 / 2.0);
	let quat = Quat::from_euler(EulerRot::XYZ, PI / 16.0, PI / 8.0, PI / 8.0);

	rotate_mesh(&mut mesh, quat, mesh_center);

	display_buf.draw_mesh(&mesh, LIGHT_DIRECTION);
	println!("{}", display_buf);
}
