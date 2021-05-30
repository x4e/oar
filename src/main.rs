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
use self::event::Event;
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
	
	loop {
		if let Ok(event) = inp_recv.recv() {
			match event {
				Event::Quit => break,
				_ => {},
			}
			
			rend_send.send(()).unwrap();
		} else {
			break
		}
	}
	
	inp_thread.join().unwrap();
	drop(rend_send);
	rend_thread.join().unwrap();
}
