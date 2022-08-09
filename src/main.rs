use std::time::{Duration, Instant};
use crate::canvas::Canvas;
use crate::shapes::donut::Donut;

mod shapes;
mod canvas;

fn main() {
    print!("\x1b[2J"); // clear screen
    print!("\x1b[?25l");// hide terminal cursor

    let canvas = Canvas::new(50, 50);
    let mut torus = Donut::new(10, 5, 15, 25, 50, 50);


    let sleep_duration = 1000 / 30; // fps 30
    let start_ts = Instant::now();

    loop {
        canvas.buffer_2_screen_raw(&torus.next_frame_with_xy_rotate(true));
        std::thread::sleep(Duration::from_millis(sleep_duration));

        if start_ts.elapsed().as_secs() >= 10 {
            break;
        }
    }

    print!("\x1b[?25h");  // show cursor
}
