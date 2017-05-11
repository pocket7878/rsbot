extern crate rsbot;

fn main() {
	let display = rsbot::open_display(None);
	let root = rsbot::root_window(display, 0);

	rsbot::set_mouse_pos(display, root, 0, 0); // Absolutely positioned
	rsbot::move_mouse(display, 50, 50);        // Relatively positioned
}
