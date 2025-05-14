#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod my_window;

use winsafe::{prelude::*, co, AnyResult, HWND, GetCursorPos, SetCursorPos};
use my_window::MyWindow;

use std::thread;
use std::fmt::Write as _;
use std::time::Duration;

fn move_mouse(x: i32, y: i32) -> Result<(), String> {
    match SetCursorPos(x, y) {
        Ok(_) => Ok(()),
        Err(e) => handle_error(e, "SetCursorPos failed")
    }
}

fn get_mouse_pos() -> Result<(i32, i32), String> {
     match GetCursorPos() {
        Ok(p) => Ok((p.x, p.y)),
        Err(e) => handle_error(e, "GetCursorPos failed")
     }
}

fn handle_error<T>(e: co::ERROR, msg:&str) -> Result<T, String> {
    let mut s = String::new();
    write!(&mut s, "{} {}", msg, e).unwrap();
    Err(s)
}

const SHIFT: i32 = 2;
const FAST_SEC: u64 = 5;
const SLOW_SEC: u64 = 30;

fn handle_mouse_coords(x: i32, y: i32, zig: bool, secs: u64) -> bool {
    let shift = if zig { SHIFT * -1 } else { SHIFT };
    let (x, y) = (x + shift, y + shift);
    match move_mouse(x, y) {
        Ok(()) => println!(
            "Moved mouse to: ({x}, {y}), {}",
            if zig { "zig" } else { "zag" }
        ),
        Err(s) => println!("{}", s),
    }

    thread::sleep(Duration::from_secs(secs));
    !zig
}

fn calc_dist(prev_x: i32, prev_y: i32, x: i32, y: i32) -> i32 {
    (((prev_x - x) * (prev_x - x) + (prev_y - y) * (prev_y - y)) as f32).sqrt() as i32
}

fn main1() {
    let mut zig: bool = true;
    let (mut prev_x, mut prev_y) = get_mouse_pos().unwrap_or_default();
    loop {
        match get_mouse_pos() {
            Ok((x, y)) => {
                let distance = calc_dist(prev_x, prev_y, x, y);
                println!("distance {distance}");
                (prev_x, prev_y) = (x, y);
                let secs = if distance <= 2 { FAST_SEC } else { SLOW_SEC };
                zig = handle_mouse_coords(x, y, zig, secs);
            }
            Err(e) => {
                println!("{}", e);
                return;
            }
        }
    }
}



fn main() {
	if let Err(e) = run_app() {
		HWND::NULL.MessageBox(
			&e.to_string(), "Uncaught error", co::MB::ICONERROR).unwrap();
	}
}

fn run_app() -> AnyResult<i32> {
	MyWindow::new() // create our main window...
		.run()       // ...and run it
		.map_err(|err| err.into())
}