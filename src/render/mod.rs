pub mod buffer;
pub mod renderable;
pub mod renderer;

pub use renderable::Renderable;
pub use renderer::start_render_thread;


use termion::cursor;

/// Returns an ansi escape sequence to move the cursor to the given 0 based 
/// usize indexes.
/// This will convert them to u16s and make them 1 based.
fn goto(x: usize, y: usize) -> cursor::Goto {
	cursor::Goto(x as u16 + 1, y as u16 + 1)
}
