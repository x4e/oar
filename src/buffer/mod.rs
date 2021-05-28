
use std::fs::File;
use std::vec::Vec;
use std::path::Path;
use std::string::String;
use std::io;

use super::utils::read::read_lines;

pub struct Buffer {
	/// The contents of the buffer, a vector of utf-8 strings, each entry being
	/// rendered as a line.
	pub lines: Vec<String>,
	/// The current position that we are viewing. This is **NOT** the location
	/// of the cursor, this is the location of the top left character that we
	/// will render on the screen.
	pub position: (usize, usize),
	/// The location of the character that the cursor is currently on top of.
	pub cursor: (usize, usize),
}

impl Buffer {
	
	pub fn from_file<P: AsRef<Path>>(path: P, crlf: bool) -> io::Result<Self> {
		let mut reader = io::BufReader::new(File::open(path)?);
		let lines = read_lines(&mut reader, crlf)?;
		
		Ok(Buffer::from_lines(lines))
	}
	
	pub fn from_lines(lines: Vec<String>) -> Self {
		Buffer {
			lines,
			position: (0, 0),
			cursor: (0, 0),
		}
	}
	
	pub fn get_line(&self, num: usize) -> Option<&String> {
		self.lines.get(num)
	}
	
	pub fn get_line_mut(&mut self, num: usize) -> Option<&mut String> {
		self.lines.get_mut(num)
	}
	
	/// Set the line at the given index to the new string, returning the old
	/// string if this operation actually replaced a string, and None if this
	/// operation did nothing.
	pub fn set_line(&mut self, num: usize, new: String) -> Option<String> {
		self.get_line_mut(num)
			.map(|item| std::mem::replace(item, new))
	}
}
