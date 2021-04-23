extern crate termion;

use termion::raw::{RawTerminal, IntoRawMode};
use termion::input::MouseTerminal;
use termion::screen::*;
use termion::terminal_size;

use std::io::{Write};

pub struct Screen <W: Write> {
	/// The terminal that is the output for this screen
	inner: MouseTerminal<RawTerminal<W>>
}

impl <W: Write> Screen<W> {
	pub fn new(out: W) -> std::io::Result<Screen<W>> {
		Ok(Screen {
			inner: MouseTerminal::from(out.into_raw_mode()?)
		})
	}
	
	pub fn switch_to_main(&mut self) -> std::io::Result<()> {
		write!(self, "{}", ToMainScreen)?;
		Ok(())
	}

	pub fn switch_to_alternate(&mut self) -> std::io::Result<()> {
		write!(self, "{}", ToAlternateScreen)?;
		Ok(())
	}
	
	pub fn clear(&mut self) -> std::io::Result<()> {
		write!(self, "{}", termion::clear::All)?;
		Ok(())
	}
	
	/// Returns 0 indexed size of terminal
	pub fn size(&mut self) -> std::io::Result<(usize, usize)> {
		let size = terminal_size()?;
		Ok((size.0 as usize - 1, size.1 as usize - 1))
	}
}

impl <W: Write> Drop for Screen<W> {
	fn drop(&mut self) {
		// ensure that we dont leave the terminal in alternate state
		let _ = self.switch_to_main();
	}
}

impl <W: Write> Write for Screen<W> {
	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
		self.inner.write(buf)
	}
	
	fn flush(&mut self) -> std::io::Result<()> {
		self.inner.flush()
	}
}
