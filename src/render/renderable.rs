
use super::screen::Screen;

use std::io::{self, Write};


/// A trait that is able to be renderable to a screen
pub trait Renderable {
	/// Render self onto a writable screen
	fn render_to_screen<W: Write>(&self, screen: &mut Screen<W>) -> io::Result<()>;
}
