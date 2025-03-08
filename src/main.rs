use std::path::{Path, PathBuf};

struct Image<'a> {
    width: i32,
    height: i32,
    data: String,
    out_dest: &'a Path
}

impl<'a> Image<'a> {


    fn draw(&self) {

    }



    fn clear_data(&mut self) {
        self.data.clear();
    }



    fn new(w: i32, h: i32, path: &'a str) -> Self {
        Self {
            width: w,
            height: h,
            data: String::new(),
            out_dest: Path::new(path)
        }
    }
}

fn main() {
    let img = Image::new(256, 256, "/foo.txt");
 
}
