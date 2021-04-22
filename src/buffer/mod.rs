
use std::fs::File;
use std::vec::Vec;
use std::path::Path;
use std::string::String;
use std::io::{self, BufRead};

pub struct Buffer {
	/// The contents of the buffer, a vector of utf-8 strings, each entry being
	/// rendered as a line.
	lines: Vec<String>,
	/// The current position that we are viewing. This is **NOT** the location
	/// of the cursor, this is the location of the top left character that we
	/// will render on the screen.
	position: (usize, usize),
	/// The location of the character that the cursor is currently on top of.
	cursor: (usize, usize),
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
	
	pub fn set_line(&mut self, num: usize, new: String) -> Option<String> {
		if let Some(item) = self.get_line_mut(num) {
			Some(std::mem::replace(item, new))
		} else {
			None
		}
	}
	
}


/// Read each line from the given buffer with respect for the given 
fn read_lines<R: BufRead + ?Sized>(r: &mut R, crlf: bool) -> io::Result<Vec<String>> {
	let ending_size = if crlf { 2 } else { 1 };
	
	let mut lines: Vec<String> = Vec::new();
	let mut line = String::new();
	
	let mut more = true;
	while more {
		if r.read_line(&mut line)? > 0 {
			// Read some data into the string - if it is a complete line (with
			// respect to crlf) then add it as a line and start a new line
			let len = line.len();
			if len < 1 || (crlf && len < 2) {
				// not enough size to contain a line ending
				continue
			}
			let bytes = &line.as_bytes();
			if bytes[len-1] != b'\n' || (crlf && bytes[len-2] != b'\r') {
				// does not end with \n or \r\n
				continue
			}
			// Remove the line ending from the string
			line.truncate(len - ending_size);
		} else {
			// Parsed whole buffer
			more = false;
		}
		
		lines.push(std::mem::replace(&mut line, String::new()));
	}
	Ok(lines)
}

#[test]
fn test_read_lines_lf() {
	// Trailing new lines
	let mut string = "\nHello\nHello\n\nHello\n\n\n".as_bytes();
	let lines = read_lines(&mut string, false).unwrap();
	assert_eq!(lines, vec!["", "Hello", "Hello", "", "Hello", "", "", ""]);
	
	// No trailing new line
	let mut string = "Hello\nHello".as_bytes();
	let lines = read_lines(&mut string, false).unwrap();
	assert_eq!(lines, vec!["Hello", "Hello"]);
}

#[test]
fn test_read_lines_crlf() {
	// Trailing new lines
	let mut string = "\r\nHello\r\nH\nello\r\n\n\r\nHello\r\n\r\n\r\n".as_bytes();
	let lines = read_lines(&mut string, true).unwrap();
	assert_eq!(lines, vec!["", "Hello", "H\nello", "\n", "Hello", "", "", ""]);
	
	// No trailing new line
	let mut string = "Hello\r\nHello".as_bytes();
	let lines = read_lines(&mut string, true).unwrap();
	assert_eq!(lines, vec!["Hello", "Hello"]);	
}
