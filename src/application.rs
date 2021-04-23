
use super::buffer::Buffer;
use super::screen::Screen;

/// Holds the state of the running application.
/// This includes any buffers that are open as well as the screen we are
/// displaying on.
pub struct Application {
	pub screen: Screen,
	pub buffers: Vec<Buffer>,
	selected_buffer: usize
}

impl Application {
	pub fn selected_buffer(&self) -> Option<Buffer> {
		let len = self.buffers.len();
		// If the selected buffer is out of bounds, truncuate it
		if self.selected_buffer >= len {
			self.selected_buffer = len - 1;
		}
		// Might still be None if there are no available buffers, so still return
		// an option
		self.buffers.get(self.selected_buffer)
	}
}
