
use crate::buffer::Buffer;

use std::sync::RwLock;


/// Holds the state of the running application.
/// This includes any buffers that are open as well as the screen we are
/// displaying on.
pub struct Application {
	pub buffers: Vec<RwLock<Buffer>>,
	selected_buffer: usize
}

impl Application {
	pub fn new() -> Self {
		Application {
			buffers: Vec::new(),
			selected_buffer: 0
		}
	}
	
	pub fn add_buffer(&mut self, buffer: Buffer) {
		self.buffers.push(RwLock::new(buffer));
	}
	
	pub fn selected_buffer(&self) -> Option<&RwLock<Buffer>> {
		let len = self.buffers.len();
		let mut selected = self.selected_buffer;
		
		// If the selected buffer is out of bounds, truncuate it
		if len > 0 && selected >= len {
			selected = len - 1;
		}
		
		self.buffers.get(selected)
	}
}

impl Default for Application {
	fn default() -> Self {
		Self::new()
	}
}
