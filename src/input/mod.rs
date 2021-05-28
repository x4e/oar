
use termion::event::{Event, Key};
use termion::input::TermRead;
use std::io;

/// This method will continously block the current thread.
/// 
/// The method will return if the user signals through input that they want
/// to quit the application.
pub fn poll_input<R: TermRead + io::Read>(read: R) {
	for e in read.events() {
		println!("{:?}", e);
		if let Ok(e) = e {
			match e {
				Event::Key(Key::Char('q')) => break,
				Event::Key(Key::Char('c')) => continue,
				_ => {}
			}
		}
	}
}
