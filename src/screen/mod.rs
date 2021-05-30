extern crate termion;

use termion::cursor;
use termion::raw::{RawTerminal, IntoRawMode};
use termion::input::MouseTerminal;
use termion::screen::*;
use termion::terminal_size;

use std::io::{self, Write};

pub struct Screen <W: Write> {
	/// The terminal that is the output for this screen
	inner: MouseTerminal<AlternateScreen<RawTerminal<W>>>
}

unsafe impl <W: Write> Send for Screen<W> {}
unsafe impl <W: Write> Sync for Screen<W> {}

impl <W: Write> Screen<W> {
	pub fn new(out: W) -> io::Result<Screen<W>> {
		Ok(Screen {
			inner: MouseTerminal::from(AlternateScreen::from(out.into_raw_mode()?))
		})
	}
	
	pub fn show_cursor(&mut self) -> io::Result<()> {
		write!(self, "{}", cursor::Show)?;
		Ok(())
	}
	
	pub fn hide_cursor(&mut self) -> io::Result<()> {
		write!(self, "{}", cursor::Hide)?;
		Ok(())
	}
	
	pub fn clear(&mut self) -> io::Result<()> {
		write!(self, "{}", termion::clear::All)?;
		Ok(())
	}
	
	/// Returns 0 indexed size of terminal
	pub fn size(&mut self) -> io::Result<(usize, usize)> {
		let size = terminal_size()?;
		Ok((size.0 as usize - 1, size.1 as usize - 1))
	}
}

impl <W: Write> Drop for Screen<W> {
	fn drop(&mut self) {
		println!("Screen dropping");
		// ensure that we dont leave the terminal in alternate state
		//_ = self.switch_to_main();
		//let _ = self.show_cursor();
		//let _ = self.flush();
		println!("Screen dropped");
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
