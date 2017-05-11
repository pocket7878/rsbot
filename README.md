# rsbot

With a bunch of help from my buddy [@tbodt](https://github.com/tbodt), I present you...  
Rsbot!

You can now press a simulate a keypresses from Rust!
```Rust
extern crate rsbot;

fn main() {
	std::thread::sleep(std::time::Duration::from_secs(1));
	let bot = rsbot::new_bot();

	bot.type_keys("hello");
}
```
Or even move your mouse!  
```Rust
extern crate rsbot;

fn main() {
	let mut bot = rsbot::new_bot();
	// set_mouse_pos needs bot to be mutable under Linux.

	bot.set_mouse_pos(0, 0); // Absolutely positioned
	bot.move_mouse(50, 50);  // Relatively positioned
}
```

With a lot of room for improvment!  

## Intentions

I knew that I would never be able to bind the whole X11 library.  
It's such a tedious task.  
I also know that I wouldn't be able to make it cross platform.

However, I honestly believe that with your help, we can do that.  
**Make pull requests! File issues! We can do this!**

## Requirements

For now, only Linux with X11 is supported.  
*I have no Windows machine.*

However, I appreciate pull requests.

