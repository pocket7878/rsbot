extern crate rsbot;

fn main() {
	std::thread::sleep(std::time::Duration::from_secs(1));
	let display = rsbot::open_display(None);

	rsbot::type_keys(display, "hello");
}
