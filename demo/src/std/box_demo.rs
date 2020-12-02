use std::mem;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    // allocate on heap
    Box::new(Point { x: 0.0, y: 0.0 })
}

pub fn demo() {
    let point: Point = origin();
    println!("{:?}", point);

    let boxed_point: Box<Point> = boxed_origin();
    println!("{:?}", boxed_point)
}
