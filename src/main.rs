mod shapes;
mod canvas;

fn main() {
    print!("\x1b[2J"); // clear screen
    print!("\x1b[?25l");// hide terminal cursor



    print!("\x1b[?25h");  // show cursor
}
