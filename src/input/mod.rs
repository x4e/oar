
use termion::event::{Event, Key};
use termion::input::TermRead;
use std::io;
use std::sync::{Arc, Condvar, Mutex};

/// This method will continously block the current thread.
/// 
/// The method will return if the user signals through input that they want
/// to quit the application.
pub fn poll_input<R: TermRead + io::Read>(read: R, sync: Arc<(Mutex<bool>, Condvar)>) {
	let (lock, cvar) = &*sync;
	let mut notify = lock.lock().unwrap();
	*notify = true;
	
	for e in read.events().flatten() {
		match e {
			Event::Key(Key::Char('q')) => {
				cvar.notify_one();
				break;
			},
			Event::Key(Key::Char('c')) => continue,
			Event::Key(Key::Up) => continue,
			Event::Key(Key::Down) => continue,
			_ => {}
		}
	}
}
