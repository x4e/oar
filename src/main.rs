extern crate termion;

pub mod buffer;
pub mod input;
pub mod render;
pub mod screen;
mod utils;

use self::buffer::Buffer;
use self::render::Renderable;
use self::screen::Screen;

use std::io::{Write, stdout, stdin};
use std::env;
use std::process::exit;

fn main() {
	
	let stdin = stdin();
	let stdout = stdout();
	
	let args: Vec<String> = env::args().collect();
	
	let buffer = if let Some(file) = args.get(1).clone() {
		println!("Reading {}", file);
		Buffer::from_file(file, false).unwrap()
	} else {
		eprintln!("Usage: oar file");
		exit(1);
	};
	
	let mut screen = Screen::new(stdout).unwrap();
	// Note: No need to switch back to main on application exit as Screen::drop
	// automatically does this for us.
	screen.switch_to_alternate().unwrap();
	
	write!(screen, "{}", termion::cursor::Hide).unwrap();
	
	buffer.render_to_screen(&mut screen).unwrap();
	screen.flush().unwrap();
	
	input::poll_input(stdin);
	
	write!(screen, "{}", termion::cursor::Show).unwrap();
}
