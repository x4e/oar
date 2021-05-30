pub mod application;
pub mod buffer;
pub mod input;
pub mod render;
pub mod screen;
mod utils;

use self::application::Application;
use self::buffer::Buffer;
use self::render::start_render_thread;

use std::io::stdin;
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
	
	let mut app = Application::new();
	app.add_buffer(buffer);
	let app = Arc::new(app);
	
	let renderer = start_render_thread(
		Arc::clone(&app)
	);
	renderer.send(()).unwrap();
	
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
		renderer.send(()).unwrap();
	}
	
	println!("Ending");
}
