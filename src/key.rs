extern crate libc;
use std;
use Display;

type Bool = libc::c_int;
type KeySym = *const libc::c_void;
type KeyCode = libc::c_uint;

#[link(name = "X11")]
extern {
	fn XStringToKeysym(string: *const std::os::raw::c_char) -> KeySym;
	fn XKeysymToKeycode(display: Display, keysym: KeySym, index: libc::c_int) -> KeyCode;
}

#[link(name = "Xtst")]
extern {
	fn XTestFakeKeyEvent(display: Display, keycode: KeyCode, state: Bool, delay: libc::c_ulong);
}

pub fn string_to_keysym(string: &str) -> KeySym {
	unsafe {
		XStringToKeysym(std::ffi::CString::new(string).unwrap().as_ptr())
	}
}
pub fn keysym_to_keypress(display: Display, keysym: KeySym, index: i32) -> KeyCode {
	unsafe {
		XKeysymToKeycode(display, keysym, index as libc::c_int)
	}
}
pub fn string_to_keycode(display: Display, string: &str) -> KeyCode {
	keysym_to_keypress(display, string_to_keysym(string), 0)
}

pub fn set_key_state(display: Display, keycode: KeyCode, state: bool) {
	unsafe {
		XTestFakeKeyEvent(display, keycode, if state { 1 as Bool } else { 0 as Bool }, 0 as libc::c_ulong);
	}
	super::flush(display)
}
pub fn push_key(display: Display, string: &str) {
	let keycode = string_to_keycode(display, string);

	set_key_state(display, keycode, true);
	set_key_state(display, keycode, false);
}
pub fn type_keys(display: Display, string: &str) {
	for c in string.chars() {
		push_key(display, c.to_string().as_str());
	}
}
