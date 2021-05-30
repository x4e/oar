
use crate::error::*;

use std::convert::TryFrom;
use std::io::{self, Write};

use crossterm::cursor;
use crossterm::terminal::{
	self,
	Clear,
	ClearType,
	EnterAlternateScreen,
	LeaveAlternateScreen
};
use crossterm::QueueableCommand;


pub struct Screen <W: Write> {
	/// The terminal that is the output for this screen
	inner: W
}

impl <W: Write> Screen<W> {
	pub fn new(inner: W) -> Screen<W> {
		Screen {
			inner
		}
	}
	
	pub fn enable_raw_mode(&mut self) -> Result<()> {
		terminal::enable_raw_mode()?;
		Ok(())
	}
	
	pub fn disable_raw_mode(&mut self) -> Result<()> {
		terminal::disable_raw_mode()?;
		Ok(())
	}
	
	pub fn switch_to_main(&mut self) -> Result<()> {
		self.queue(EnterAlternateScreen)?;
		Ok(())
	}
	
	pub fn switch_to_alternate(&mut self) -> Result<()> {
		self.queue(LeaveAlternateScreen)?;
		Ok(())
	}
	
	pub fn show_cursor(&mut self) -> Result<()> {
		self.queue(cursor::Show)?;
		Ok(())
	}
	
	pub fn hide_cursor(&mut self) -> Result<()> {
		self.queue(cursor::Hide)?;
		Ok(())
	}
	
	/// Returns 0 indexed size of terminal
	pub fn size(&mut self) -> Result<(usize, usize)> {
		let size = terminal::size()?;
		Ok((
			usize::try_from(size.0)? - 1,
			usize::try_from(size.1)? - 1
		))
	}
	
	pub fn goto(&mut self, x: usize, y: usize) -> Result<()> {
		self.queue(cursor::MoveTo(
			u16::try_from(x)?,
			u16::try_from(y)?
		))?;
		Ok(())
	}
	
	pub fn clear(&mut self) -> Result<()> {
		self.queue(Clear(ClearType::All))?;
		Ok(())
	}
}

impl <W: Write> Write for Screen<W> {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		self.inner.write(buf)
	}
	
	fn flush(&mut self) -> io::Result<()> {
		self.inner.flush()
	}
}
