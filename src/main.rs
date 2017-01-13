extern crate image;

extern crate wavefront_obj;
use wavefront_obj::obj::Primitive;

mod img;
use img::Img;

fn main() {
    let w = 1000;
    let h = 1000;
    let mut img = Img::new(w, h);

    let file = wavefront_obj::obj::parse(include_str!("../head.wobj").to_string());
    let obj = file.unwrap().objects.into_iter().next().unwrap();
    let ref geometries = obj.geometry;
    let ref vertices = obj.vertices;
    for geometry in geometries {
        for shape in &geometry.shapes {
            match shape.primitive {
                Primitive::Triangle(a, b, c) => {
                    let (a, b, c) = (vertices[a.0], vertices[b.0], vertices[c.0]);
                    for &(start, end) in &[(a,b), (b,c), (c,a)] {
                        let dim = (
                            ((start.y - 1.0) * (h as f64) / -2.0) as i64,
                            ((start.x + 1.0) * (h as f64) / 2.0) as i64,
                            ((end.y - 1.0) * (h as f64) / -2.0) as i64,
                            ((end.x + 1.0) * (h as f64) / 2.0) as i64,
                        );
                        line(&mut img, dim, (255, 255, 255));
                    }
                },
                Primitive::Line(a, b) =>  {

                },
                _ => {}
            }
            println!("{:?}", shape.primitive);
        }
    }

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

    let dx = x1 - x0;
    let dy = y1 - y0;
    let derror2 = dy.abs() * 2;
    let mut error2 = 0; // error * dx * 2

    let mut y = y0;
    for x in x0..(x1 + 1) {
        if steep {
            img.put(x as u32, y as u32, color);
        } else {
            img.put(y as u32, x as u32, color);
        }
        error2 += derror2;
        if error2 > dx {
            y += if y1 > y0 { 1 } else { -1 };
            error2 -= dx * 2;
        }
    }
}
