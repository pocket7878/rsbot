extern crate libc;

pub mod key;
pub use self::key::*;

type Display = *const libc::c_void;

#[link(name = "X11")]
extern {
	fn XOpenDisplay(string: *const std::os::raw::c_char) -> Display;
	fn XFlush(display: Display) -> libc::c_int;
}

pub fn open_display(name: Option<&str>) -> Display {
	unsafe {
		match name {
			Some(string) => {
				XOpenDisplay(std::ffi::CString::new(string).unwrap().as_ptr())
			},
			None => XOpenDisplay(std::ptr::null()),
		}
	}
}

pub fn flush(display: Display) {
	unsafe {
		XFlush(display);
	}
}

#[test]
fn it_works() {
	std::thread::sleep(std::time::Duration::from_secs(2));
	let display = open_display(None);
	self::key::type_keys(display, "hello");
}
