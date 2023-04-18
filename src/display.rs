use std::fmt::Display;

use crate::draw::DisplayBuffer;

impl<const N: usize, const M: usize> Display for DisplayBuffer<N, M> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut print_buf = String::new();
		for row in self.0 {
			for val in row {
				let pixel = if val > 128 { '#' } else { ' ' };
				print_buf.push(pixel);
				print_buf.push(pixel);
			}
			print_buf.push('\n');
		}

		write!(f, "{}", print_buf)
	}
}
