
use crate::application::Application;
use crate::error::Result;
use crate::event::Event;

use std::sync::mpsc;
use std::thread::{self, JoinHandle};

use crossterm::event::{self, Event as TermEvent, KeyCode};

/// This method will continously block the current thread.
/// 
/// The method will return if the user signals through input that they want
/// to quit the application.
pub fn start_input_thread() -> (JoinHandle<Result<()>>, mpsc::Receiver<Event>) {
	let (send, recv) = mpsc::channel::<Event>();
	
	(
		thread::spawn(move || listen_for_events(send)),
		recv
	)
}

fn listen_for_events(send: mpsc::Sender<Event>) -> Result<()> {
	loop {
		match event::read().unwrap() {
			TermEvent::Key(event) => {
				match event.code {
					KeyCode::Char('q') => {
						send.send(Event::Quit)?;
						break;
					},
					KeyCode::Char('c') => {},
					KeyCode::Up => send.send(Event::ScrollUp)?,
					KeyCode::Down => send.send(Event::ScrollDown)?,
					_ => {}
				}
			},
			TermEvent::Mouse(_event) => {},
			TermEvent::Resize(_width, _height) => {}
		}
	}
	Ok(())
}

pub enum EventResult {
	/// Quit the application
	Quit,
	/// Re-render the screen
	Render,
	/// Dont do any further actions
	None
}

/// Called from main thread
/// Returns whether a re-render is needed
pub fn process_event(app: &Application, event: Event) -> Result<EventResult> {
	Ok(match event {
		Event::Quit => EventResult::Quit,
		Event::ScrollUp => {
			let app = &*app;
			if let Some(buf) = app.selected_buffer() {
				let mut buf = buf.write()?;
				if let Some(y) = buf.position.1.checked_sub(1) {
					buf.position.1 = y;
				}
			}
			EventResult::Render
		},
		Event::ScrollDown => {
			let app = &*app;
			if let Some(buf) = app.selected_buffer() {
				let mut buf = buf.write()?;
				if let Some(y) = buf.position.1.checked_add(1) {
					buf.position.1 = y;
				}
			}
			EventResult::Render
		},
	})
}
