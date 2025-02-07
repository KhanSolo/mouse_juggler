use std::thread;
use std::time::Duration;
use winapi::shared::windef::POINT;
use winapi::um::winuser::{GetCursorPos, SetCursorPos};

fn move_mouse(x: i32, y: i32) -> Result<(), String> {
    unsafe {
        if SetCursorPos(x, y) == 0 {
            Err("SetCursorPos failed".into())
        } else {
            Ok(())
        }
    }
}

fn get_mouse_pos() -> Result<(i32, i32), String> {
    unsafe {
        let mut point = POINT { x: 0, y: 0 };
        if GetCursorPos(&mut point) == 0 {
            Err("GetCursorPos failed".into())
        } else {
            Ok((point.x, point.y))
        }
    }
}

fn handle_mouse_coords(x: i32, y: i32, zig: bool) {
    let mut shift = 2;
    if zig { shift *= -1; }

    let (x, y) = (x + shift, y + shift);
    match move_mouse(x, y) {
        Ok(()) => println!(
            "Moved mouse to: ({x}, {y}), {}",
            if zig { "zig" } else { "zag" }
        ),
        Err(s) => println!("{}", s),
    }

    thread::sleep(Duration::from_secs(5));
}

fn main() {
    let mut zig = true;
    loop {
        match get_mouse_pos() {
            Ok((x, y)) => {
                handle_mouse_coords(x, y, zig);
                zig = !zig;
            }
            Err(e) => {
                println!("{}", e);
                return;
            }
        }
    }
}
