extern crate image;

mod img;
use img::Img;

fn main() {
    let mut img = Img::new();
    img.put(10, 10, 100, 100, 100);
    img.save();
}
