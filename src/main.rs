extern crate image;

extern crate wavefront_obj;
use wavefront_obj::obj::Primitive;

mod img;
use img::Img;

fn main() {
    let w = 1000;
    let h = 1000;
    let mut img = Img::new(w, h);

    triangle(&mut img, (10, 10), (30, 30), (15, 53), (255, 255, 255));

    // let file = wavefront_obj::obj::parse(include_str!("../head.wobj").to_string());
    // let obj = file.unwrap().objects.into_iter().next().unwrap();
    // let ref geometries = obj.geometry;
    // let ref vertices = obj.vertices;
    // for geometry in geometries {
    //     for shape in &geometry.shapes {
    //         match shape.primitive {
    //             Primitive::Triangle(a, b, c) => {
    //                 let (a, b, c) = (vertices[a.0], vertices[b.0], vertices[c.0]);
    //                 for &(start, end) in &[(a,b), (b,c), (c,a)] {
    //                     let dim = (
    //                         ((start.y - 1.0) * (h as f64) / -2.0) as i64,
    //                         ((start.x + 1.0) * (h as f64) / 2.0) as i64,
    //                         ((end.y - 1.0) * (h as f64) / -2.0) as i64,
    //                         ((end.x + 1.0) * (h as f64) / 2.0) as i64,
    //                     );
    //                     line(&mut img, dim, (255, 255, 255));
    //                 }
    //             },
    //             Primitive::Line(a, b) =>  {

    //             },
    //             _ => {}
    //         }
    //         println!("{:?}", shape.primitive);
    //     }
    // }

    img.save();
}

fn triangle(img: &mut Img, t0: (i64, i64), t1: (i64, i64), t2: (i64, i64), color: (u8,u8,u8)) {
    let mut vertices = [t0, t1, t2];;
    vertices.sort_by(|a, b| (a.1).cmp(&b.1));
    let mut v_iter = vertices.into_iter();
    let &(x0, y0) = v_iter.next().unwrap();
    let &(x1, y1) = v_iter.next().unwrap();
    let &(x2, y2) = v_iter.next().unwrap();

    let mut start_x = x0; // travels directly to x2
    let start_dx = x2 - x0; // total horizontal pixels to travel
    let start_dy = y2 - y0; // total vertical pixels to travel
    let mut start_error = 0; // error * dy * 2
    let start_error_step = (x2 - x0).abs() * 2; // change in error per vertical pixel

    let mut end_x = x0;
    let end_dx = x1 - x0; // total horizontal pixels to travel
    let end_dy = y1 - y0; // total vertical pixels to travel
    let mut end_error = 0; // error * dy * 2
    let end_error_step = (x1 - x0).abs() * 2; // change in error per vertical pixel

    for y in y0..(y1+1) {
        line(img, (start_x, y), (end_x, y), color);
        start_error += start_error_step;
        if start_error > start_dy {
            start_x += if start_dx > 0 { 1 } else { -1 };
            start_error -= start_dy * 2;
        }
        end_error += end_error_step;
        if end_error > end_dy {
            end_x += if end_dx > 0 { 1 } else { -1 };
            end_error -= end_dy * 2;
        }
    }

    let mut end_x = x1;
    let end_dx = x2 - x1; // total horizontal pixels to travel
    let end_dy = y2 - y1; // total vertical pixels to travel
    let mut end_error = 0; // error * dy * 2
    let end_error_step = (x2 - x1).abs() * 2; // change in error per vertical pixel

    for y in y1..(y2+1) {
        line(img, (start_x, y), (end_x, y), color);
        start_error += start_error_step;
        if start_error > start_dy {
            start_x += if start_dx > 0 { 1 } else { -1 };
            start_error -= start_dy * 2;
        }
        end_error += end_error_step;
        if end_error > end_dy {
            end_x += if end_dx > 0 { 1 } else { -1 };
            end_error -= end_dy * 2;
        }
    }
    line(img, t0, t1, (255,0,0));
    line(img, t2, t1, (255,0,0));
    line(img, t0, t2, (255,0,0));
}

fn line(img: &mut Img, t0: (i64, i64), t1: (i64, i64), color: (u8, u8, u8)) {
    let (x0, y0) = t0;
    let (x1, y1) = t1;

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
