extern crate libc;
use {Display, Window};

#[link(name = "X11")]
extern {
	fn XWarpPointer(display: Display, src_w: Window, dest_w: Window,
					src_x: libc::c_int, src_y: libc::c_int,
					src_width: libc::c_int, src_height: libc::c_int,
					dest_x: libc::c_int, dest_y: libc::c_int);
}

pub fn set_mouse_pos(display: Display, rel: Window, x: i32, y: i32) {
	unsafe {
		XWarpPointer(display, 0 as Window, rel,
					 0 as libc::c_int, 0 as libc::c_int,
					 0 as libc::c_int, 0 as libc::c_int,
					 x as libc::c_int, y as libc::c_int);
	}
	super::flush(display);
}

pub fn move_mouse(display: Display, x: i32, y: i32) {
	set_mouse_pos(display, 0 as Window, x, y);
}
