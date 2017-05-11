extern crate libc;

pub mod key;
pub mod mouse;
pub use self::key::*;
pub use self::mouse::*;

type Display = *const libc::c_void;
type Window = libc::c_int;

#[link(name = "X11")]
extern {
	fn XOpenDisplay(string: *const std::os::raw::c_char) -> Display;
	fn XFlush(display: Display) -> libc::c_int;
	fn XRootWindow(display: Display, index: libc::c_int) -> Window;
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

pub fn root_window(display: Display, index: i32) -> Window {
	unsafe {
		XRootWindow(display, index as libc::c_int)
	}
}
