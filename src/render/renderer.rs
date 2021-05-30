use crate::application::Application;
use crate::error::*;
use crate::screen::Screen;
use super::Renderable;

use std::io::{stdout, Write};
use std::sync::{Arc, mpsc};
use std::thread::{self, JoinHandle};

pub fn start_render_thread(app: Arc<Application>) -> (JoinHandle<Result<()>>, mpsc::Sender<()>) {
	let (send, recv) = mpsc::channel::<()>();
	
	(
		thread::spawn(move || render(app, recv)),
		send
	)
}

fn render(app: Arc<Application>, recv: mpsc::Receiver<()>) -> Result<()> {
	let mut screen = Screen::new(stdout());
	// Get underlying app from arc
	let app = &*app;
	
	screen.switch_to_alternate()?;
	screen.enable_raw_mode()?;
	screen.hide_cursor()?;
	screen.clear()?;
	screen.flush()?;
	
	loop {
		if recv.recv().is_err() {
			// The sender has disconnected, shut down the thread
			break
		}
		// The sender has sent us something which is our cue to render
		
		if let Some(buf) = app.selected_buffer() {
			let buf = buf.read()?;
			buf.render_to_screen(&mut screen).unwrap();
		}
	}
	
	// cleanup screen state
	screen.show_cursor()?;
	screen.switch_to_main()?;
	screen.disable_raw_mode()?;
	screen.flush()?;
	
	Ok(())
}
