use std::u32;

use winapi::um::winuser::{mouse_event, MOUSEEVENTF_MOVE};

fn main() {
	print!("This application keeps the machine awake!\n");
	loop {
		unsafe {
			mouse_event(MOUSEEVENTF_MOVE, 0, 1, 0, 0);
			std::thread::sleep(std::time::Duration::from_secs(1));
			mouse_event(MOUSEEVENTF_MOVE, 0, u32::MAX, 0, 0);
			std::thread::sleep(std::time::Duration::from_secs(1));
		}
	}
}
