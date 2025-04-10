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

const SHIFT: i32 = 2;

fn handle_mouse_coords(x: i32, y: i32, zig: bool, secs: u64) {
    let mut shift = SHIFT;
    if zig {
        shift *= -1;
    }

    let (x, y) = (x + shift, y + shift);
    match move_mouse(x, y) {
        Ok(()) => println!(
            "Moved mouse to: ({x}, {y}), {}",
            if zig { "zig" } else { "zag" }
        ),
        Err(s) => println!("{}", s),
    }

    thread::sleep(Duration::from_secs(secs));
}

fn calc_dist(prev_x: i32, prev_y: i32, x: i32, y: i32) -> i32 {
    (((prev_x - x) * (prev_x - x) + (prev_y - y) * (prev_y - y)) as f32).sqrt() as i32
}

fn main() {
    let mut zig = true;
    let (mut prev_x, mut prev_y) = (0, 0);
    loop {
        match get_mouse_pos() {
            Ok((x, y)) => {
                let distance = calc_dist(prev_x, prev_y, x, y);
                println!("distance {distance}");
                (prev_x, prev_y) = (x, y);
                let secs = if distance == 2 { 5 } else { 30 };
                handle_mouse_coords(x, y, zig, secs);
                zig = !zig;
            }
            Err(e) => {
                println!("{}", e);
                return;
            }
        }
    }
}
