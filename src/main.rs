use std::path::{Path, PathBuf};

struct Image {
    width: i32,
    height: i32,
    data: String,
    out_dest: PathBuf
}

impl Image {


    fn draw(&self) {

    }



    fn clear_data(&mut self) {
        self.data.clear();
    }



    fn new(w: i32, h: i32) -> Self {
        Self {
            width: w,
            height: h,
            data: String::new(),
            out_dest: PathBuf::new()
        }
    }
}

fn main() {
    let img = Image::new(256, 256);
 
}
