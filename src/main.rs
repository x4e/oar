pub mod application;
pub mod buffer;
pub mod error;
pub mod event;
pub mod input;
pub mod render;
pub mod screen;
mod utils;

use self::application::Application;
use self::buffer::Buffer;
use self::input::EventResult;
use self::render::start_render_thread;

use std::env;
use std::process::exit;
use std::sync::{Arc};

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
	
	let (rend_thread, rend_send) = start_render_thread(
		Arc::clone(&app)
	);
	rend_send.send(()).unwrap();
	
	let (inp_thread, inp_recv) = input::start_input_thread();
	
	while let Ok(event) = inp_recv.recv() {
		match input::process_event(&*app, event) {
			Ok(EventResult::Quit) => break,
			Ok(EventResult::Render) => rend_send.send(()).unwrap(),
			Ok(EventResult::None) => {},
			Err(e) => eprintln!("{:?}", e),
		}
	}
	
	// Make sure the other threads have a chance to shutdown
	inp_thread.join().unwrap().unwrap();
	drop(rend_send);
	rend_thread.join().unwrap().unwrap();
}
