struct DisplayBuffer<const N: usize, const M: usize>(pub [[u8; M]; N]);

fn main() {
	let mut display_buf = DisplayBuffer([[0; 12]; 12]);

	display_buf.0[0][0] = u8::MAX;
	display_buf.0[0][10] = u8::MAX;
	display_buf.0[10][0] = u8::MAX;
	display_buf.0[10][10] = u8::MAX;

	let mut print_buf = String::new();
	for row in display_buf.0 {
		for val in row {
			let pixel = if val > 128 { '#' } else { ' ' };
			print_buf.push(pixel);
			print_buf.push(pixel);
		}
		print_buf.push('\n');
	}

	println!("{}", print_buf);
}
