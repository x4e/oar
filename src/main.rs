extern crate termion;

pub mod buffer;
pub mod screen;

use self::screen::Screen;
use self::buffer::Buffer;

use termion::event::Key;
use termion::input::TermRead;
use std::io::{Write, stdout, stdin};
use std::env;

fn main() {
	let stdin = stdin();
	let stdout = stdout();
	
	let mut screen = Screen::new(stdout).unwrap();
	screen.switch_to_alternate().unwrap();

	let args: Vec<String> = env::args().collect();	
	let buffer = Buffer::from_file(args[0].clone(), false);
	
	write!(screen, "{}", termion::cursor::Hide).unwrap();
	write!(screen, "Welcome to alternate screen").unwrap();
		
	screen.flush().unwrap();
	
	for c in stdin.keys() {
		match c.unwrap() {
			Key::Char('q') => break,
			Key::Char('1') => {
				screen.switch_to_main().unwrap();
			}
			Key::Char('2') => {
				screen.switch_to_alternate().unwrap();
				write!(screen, "Welcome to alternate screen").unwrap();
			}
			Key::Char(x) => {
				write!(screen, "{}\n", x).unwrap();
			}
			_ => {}
		}
		screen.flush().unwrap();
	}
	write!(screen, "{}", termion::cursor::Show).unwrap();
}
