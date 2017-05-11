extern crate rsbot;

fn main() {
	std::thread::sleep(std::time::Duration::from_secs(1));
	let bot = rsbot::new_bot();

	bot.type_keys("hello");
}
