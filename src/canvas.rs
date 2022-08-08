pub struct Canvas {
    length: i32,
    width: i32,
}

impl Canvas {
    pub fn new(length: i32, width: i32) -> Self {
        return Canvas {
            length,
            width
        }
    }


    pub fn buffer_2_screen_raw(&self, pixel_buf: &Vec<Vec<char>>) {
        print!("\x1b[H");
        for y in  0 .. self.length {
            let y = y as usize;
            for x in 0 .. self.width {
                let x = x as usize;
                if pixel_buf[x][y] == '\0' {
                    print!("  ");
                }else {
                    print!("{} ", pixel_buf[x][y]);
                }
            }
            println!();
        }
    }
}