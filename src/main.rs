extern crate termion;

pub mod buffer;
pub mod input;
pub mod screen;

use self::screen::Screen;
use self::buffer::Buffer;

use termion::event::{Event, Key};
use termion::input::TermRead;
use std::io::{Write, stdout, stdin};
use std::env;

fn main() {
	
	let stdin = stdin();
	let stdout = stdout();
	
	let mut screen = Screen::new(stdout).unwrap();
	screen.switch_to_alternate().unwrap();
	
	let args: Vec<String> = env::args().collect();
	let file = args[1].clone();
	println!("Reading {}", file);
	let mut buffer = Buffer::from_file(file, false).unwrap();
	
	write!(screen, "{}", termion::cursor::Hide).unwrap();
	
	buffer.write_to_screen(&mut screen).unwrap();
	
	screen.flush().unwrap();
	
	for e in stdin.events() {
		println!("{:?}", e);
		if let Ok(e) = e {
			match e {
				Event::Key(Key::Char('q')) => break,
				Event::Key(Key::Char('c')) => continue,
				_ => {}
			}
		}
	}
	//for c in stdin.keys() {
	//	match c.unwrap() {
	//		Key::Char('q') => break,
	//		_ => {}
	//	}
	//}
	write!(screen, "{}", termion::cursor::Show).unwrap();
}
