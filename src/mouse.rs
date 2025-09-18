use std::fmt::Write;
use winsafe::{/*self as w,*/ co, /*gui,*/ GetCursorPos, SetCursorPos};

pub fn move_mouse(x: i32, y: i32) -> Result<(), String> {
    match SetCursorPos(x, y) {
        Ok(_) => Ok(()),
        Err(e) => handle_error(e, "SetCursorPos failed"),
    }
}

pub fn get_mouse_pos() -> Result<(i32, i32), String> {
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