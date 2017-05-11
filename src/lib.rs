mod linux;

pub fn new_bot() -> Bot {
	Bot {
		display:     linux::open_display(None),
		root_window: None,
	}
}

pub struct Bot {
	display:     linux::Display,
	root_window: Option<linux::Window>,
}
impl Bot {
	pub fn push_key(&self, string: &str) {
		linux::push_key(self.display, string);
	}
	pub fn type_keys(&self, string: &str) {
		linux::type_keys(self.display, string);
	}

	pub fn set_mouse_pos(&mut self, x: i32, y: i32) {
		if self.root_window.is_none() {
			self.root_window = Some(linux::root_window(self.display, 0));
		}
		linux::set_mouse_pos(self.display, self.root_window.unwrap(), x, y);
	}
	pub fn move_mouse(&self, x: i32, y: i32) {
		linux::move_mouse(self.display, x, y);
	}
}
