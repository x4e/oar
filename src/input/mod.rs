
use crate::error::*;
use crate::event::Event;

use std::sync::{mpsc};
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
