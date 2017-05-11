# rsbot

With a bunch of help from my buddy [@tbodt](https://github.com/tbodt),  
I bound a few functions from the X11 C library to Rust!

You can now press a simulate a keypress from Rust!  
```Rust
	let display = open_display(None);
	self::key::push_key(display, "a");
```

With a lot of room for improvment!  

## Intentions

I knew that I would never be able to bind the whole X11 library.  
It's such a tedious task.  
I also know that I wouldn't be able to make it cross platform.

However, I honestly believe that with your help, we can do that.  
Make pull requests! File issues! We can do this!

## Requirements

But yep. For now, only Linux with X11 is supported.
