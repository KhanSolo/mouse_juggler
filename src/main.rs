use std::thread;
use std::time::Duration;
use winapi::um::winuser::{GetCursorPos, SetCursorPos, GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
use winapi::shared::windef::POINT;

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

fn get_screen_resolution() -> Result<(i32, i32), String> {
    unsafe { 
        let width = GetSystemMetrics(SM_CXSCREEN);
        let height = GetSystemMetrics(SM_CYSCREEN);
        if width == 0 || height == 0 {
            Err("GetSystemMetrics failed".into())
        } else {
            Ok((width, height))
        }
    }
}

fn handle_mouse_coords(x:i32, y:i32, zig:bool, width:i32, height:i32) {
    let mut shift = 10;
    if zig { shift *= -1; }

    let x = (x + shift) % width;
    let y = (y + shift) % height;

    match move_mouse(x, y) {
        Ok(()) => println!("Moved mouse to: ({}, {}), {}", x, y, if zig { "zig" } else { "zag" }),
        Err(s) => println!("{}", s),
    }        

    thread::sleep(Duration::from_secs(5));
}

fn main() {

    match get_screen_resolution() {
        Ok((width, height)) => {
            let mut zig = true;
            loop {
                match get_mouse_pos() {
                    Ok((x,y)) => {
                        handle_mouse_coords(x, y, zig, width, height);
                        zig = !zig;
                    },
                    Err(e) => {
                        println!("{}", e);
                        return;
                    },
                }
            }
        },
        Err(e) => {
            println!("{}", e);
            return;
        },
    }
}