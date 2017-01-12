extern crate image;

mod img;
use img::Img;

fn main() {
    let mut img = Img::new();
    line(&mut img, (5, 10, 400, 40), (255, 255, 255));
    line(&mut img, (10, 5, 40, 400), (255, 0, 0));
    img.save();
}

fn line(img: &mut Img, pos: (i64, i64, i64, i64), color: (u8, u8, u8)) {
    let (x0, y0, x1, y1) = pos;

    let (x0, y0, x1, y1, steep) = if (x0 - x1).abs() < (y0 - y1).abs() {
        (y0, x0, y1, x1, true)
    } else {
        (x0, y0, x1, y1, false)
    };

    let (x0, y0, x1, y1) = if x0 > x1 {
        (x1, y1, x0, y0)
    } else {
        (x0, y0, x1, y1)
    };

    for i in x0..x1 {
        let t = ((i-x0) as f64)/((x1-x0) as f64);
        let x = x0 as f64 * (1.0-t) + x1 as f64 * t;
        let y = y0 as f64 * (1.0-t) + y1 as f64 * t;
        println!("drawing at {} {} with steep {}", x as u32, y as u32, steep);
        if steep {
            img.put(x as u32, y as u32, color);
        } else {
            img.put(y as u32, x as u32, color);
        }
    }
}
