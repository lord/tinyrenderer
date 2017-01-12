extern crate image;

mod img;
use img::Img;

fn main() {
    let mut img = Img::new();
    line(&mut img, 5, 10, 800, 29);
    img.save();
}

fn line(img: &mut Img, x0: u32, y0: u32, x1: u32, y1: u32) {
    for i in x0..x1 {
        let mut t = ((i-x0) as f64)/((x1-x0) as f64);
        let x = x0 as f64 * (1.0-t) + x1 as f64 * t;
        let y = y0 as f64 * (1.0-t) + y1 as f64 * t;
        img.put(x as u32, y as u32, 255, 255, 255);
        t += 0.01;
    }
}
