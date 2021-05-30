use crate::application::Application;
use crate::screen::Screen;
use super::Renderable;

use std::io::{Write, stdout};
use std::sync::{Arc, mpsc};
use std::thread;

pub fn start_render_thread(
	app: Arc<Application>,
) -> mpsc::Sender<()> {
	let (send, recv) = mpsc::channel::<()>();	
	
	thread::spawn(move || {
		let mut screen = Screen::new(stdout()).unwrap();
		// Get underlying app from arc
		let app = &*app;
		
		//screen.switch_to_alternate().unwrap();
		//screen.hide_cursor().unwrap();
		//screen.flush().unwrap();
		
		println!("Hello\nHello\nhello\n~\n~");
		loop {
			if recv.recv().is_err() {
				// The sender has disconnected, shut down the thread
				break
			}
			// The sender has sent us something which is our cue to render
			
			if let Some(buf) = app.selected_buffer() {
				//buf.render_to_screen(&mut screen).unwrap();
			}
		}
		
		// cleanup screen state
		//screen.switch_to_main().unwrap();
		//screen.show_cursor().unwrap();
		//screen.flush().unwrap();
		println!("Cleanup render state");
	});
	
	send
}
