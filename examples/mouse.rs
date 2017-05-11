extern crate rsbot;

fn main() {
	let mut bot = rsbot::new_bot();
	// set_mouse_pos needs bot to be mutable under Linux.

	bot.set_mouse_pos(0, 0); // Absolutely positioned
	bot.move_mouse(50, 50);  // Relatively positioned
}
