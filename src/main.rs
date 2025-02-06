use std::thread;
use std::time::Duration;
use winapi::um::winuser::{GetCursorPos, SetCursorPos, GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
use winapi::shared::windef::POINT;

fn move_mouse(x: i32, y: i32) {
    unsafe { SetCursorPos(x, y); }
}

fn get_mouse_pos() -> (i32, i32) {
    let mut point = POINT { x: 0, y: 0 };
    unsafe { GetCursorPos(&mut point as *mut POINT) };
    (point.x, point.y)
}

fn get_screen_resolution() -> (i32, i32) {
    unsafe { (GetSystemMetrics(SM_CXSCREEN), GetSystemMetrics(SM_CYSCREEN)) }
}

fn main() {
    let (width, height) = get_screen_resolution();    
    let mut zig = true;
    loop {
        let(mut x, mut y) = get_mouse_pos();

        let mut shift = 10;
        if zig { shift *= -1; }
        zig = !zig;

        x = (x + shift) % width;
        y = (y + shift) % height;

        move_mouse(x, y);
        println!("Moved mouse to: ({}, {}), zig: {}", x, y, zig);

        thread::sleep(Duration::from_secs(5));
    }
}