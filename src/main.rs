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
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn main() {
	let args: Vec<String> = env::args().collect();
	
	let buffer = if let Some(file) = args.get(1) {
		println!("Reading {}", file);
		Buffer::from_file(file, false).unwrap()
	} else {
		eprintln!("Usage: oar file");
		exit(1);
	};
	
	let mut screen = Screen::new(stdout()).unwrap();
	// Note: No need to switch back to main on application exit as Screen::drop
	// automatically does this for us.
	screen.switch_to_alternate().unwrap();
	
	write!(screen, "{}", termion::cursor::Hide).unwrap();
	
	buffer.render_to_screen(&mut screen).unwrap();
	screen.flush().unwrap();
	
	#[allow(clippy::mutex_atomic)]
	let sync = Arc::new((Mutex::new(false), Condvar::new()));
	let sync2 = Arc::clone(&sync);
	
	thread::spawn(move|| {
		input::poll_input(stdin(), sync2);
	});
	
	let (lock, cvar) = &*sync;
	let mut notify = lock.lock().unwrap();
	while !*notify {
		notify = cvar.wait(notify).unwrap();
	}
	
	write!(screen, "{}", termion::cursor::Show).unwrap();
}
