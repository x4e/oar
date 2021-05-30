use crate::Buffer;
use crate::error::*;
use crate::screen::Screen;
use super::Renderable;

use std::cmp::min;
use std::io::{Write};


impl Renderable for Buffer {
	fn render_to_screen<W: Write>(&self, screen: &mut Screen<W>) -> Result<()> {
		let pos = self.position;
		let cursor = self.cursor;
		let size = screen.size()?;
		
		screen.clear()?;
		
		let mut lines = self.lines.iter().skip(pos.1);
		
		for y in 0..size.1 {
			let line = match lines.next() {
				Some(line) => line,
				None => break
			};
			
			screen.goto(0, y)?;
			
			let x = pos.0;
			let end_x = min(line.len() - 1, x + size.0);
			let line: String = line.chars().take(end_x).skip(x).collect();
			
			write!(screen, "{}", line);
		}
		
		screen.goto(cursor.0, cursor.1)?;
		screen.flush()?;
		Ok(())
	}
}
