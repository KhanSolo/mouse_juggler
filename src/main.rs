use std::io::{self, Read};
use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use std::time::Duration;
use calc::calc_dist;
use mouse::{get_mouse_pos, move_mouse};

mod calc;
mod mouse;

const SHIFT: i32 = 2;
const THRESHOLD: i32 = 10;
const SHORT: u64 = 5;
const LONG: u64 = 30;

fn handle_mouse_coords(x: i32, y: i32, zig: bool) -> bool {
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
    !zig
}

fn main() -> Result<(), String> {
    let (stop_tx, stop_rx) = mpsc::channel::<()>();
    let handle: JoinHandle<Result<(), String>> = thread::spawn(move || {
        let mut zig = true;
        let (mut prev_x, mut prev_y) = get_mouse_pos()?;
        let mut secs = SHORT;
        loop {            
            if stop_rx.recv_timeout(Duration::from_secs(secs)).is_ok() {
                println!("Background thread completed.");
                break;
            }

            let (x, y) = get_mouse_pos()?;
            let distance = calc_dist(prev_x, prev_y, x, y);
            println!("distance {distance}");            
            (prev_x, prev_y) = (x, y);

            if distance < THRESHOLD {
                secs = SHORT;
                zig = handle_mouse_coords(x, y, zig);
            } else {
                secs = LONG
            }
        }
        Ok(())
    });

    println!("Press enter for exit...");     

    io::stdin().read(&mut [0u8]).unwrap(); // Waiting for a key press

    // Sending completion signal
    match stop_tx.send(()) {
        Ok(()) => println!("Completion signal has been sent to thread"),
        Err(e) => println!("Some error happened on sending: {e}"),
    }

    match handle.join() {
        Ok(Ok(())) => {
            println!("Thread succeeded!");
            return Ok(());
        },
        Ok(Err(e)) => {
            println!("Thread returned the error: {e}");
            return Err(e);
        },
        Err(panic) => {
            let s = format!("Thread panicked: {:?}", panic);
            println!("{s}");            
            return Err(s);
        },
    }
}
