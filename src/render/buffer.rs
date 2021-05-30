use crate::Buffer;
use crate::error::*;
use crate::screen::Screen;
use super::Renderable;

use std::io::{Write};


impl Renderable for Buffer {
	fn render_to_screen<W: Write>(&self, screen: &mut Screen<W>) -> Result<()> {
		let pos = self.position;
		let cursor = self.cursor;
		let size = screen.size()?;
		
		screen.clear()?;
		
		let mut y = pos.0;
		let lines = self.lines.iter().skip(y);
		
		for line in lines {
			if y >= (size.1 - pos.1) {
				break;
			}
			
			let mut x = pos.0;
			let chars = line.chars().skip(x);
			
			for c in chars {
				if x >= (size.0 - pos.0) {
					break;
				}
				
				screen.goto(x, y)?;
				write!(screen, "{}", c)?;
				
				x += 1;
			}
			
			y += 1;
		}
		
		// Fill remaining lines with ~
		//while y < (size.1 - pos.1) {
			
		//}
		
		screen.goto(cursor.0, cursor.1)?;
		screen.flush()?;
		Ok(())
	}
}
