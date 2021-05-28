use std::io::{self, BufRead};
use std::vec::Vec;

/// Read each line from the given buffer with respect for the given line ending format
pub fn read_lines<R: BufRead + ?Sized>(r: &mut R, crlf: bool) -> io::Result<Vec<String>> {
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
