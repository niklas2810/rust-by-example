use core::f32;

#[derive(Debug)]
struct Point(f32, f32);

#[derive(Debug)]
struct Rectangle(Point, Point);

fn rect_area(Rectangle(Point(x1, y1), Point(x2, y2)): Rectangle) -> f32 {
    let width = x2-x1;
    let height = y2-y1;
    return width*height;
}

fn square(Point(x, y): Point, size: f32) -> Rectangle {
    return Rectangle(Point(x,y), Point(x+size, y+size));
}

fn main() {
    let rect = Rectangle(Point(0.0, 0.0), Point(1.0, 2.0));

    println!("Rectangle: {:?}", rect);
    println!("Area: {}", rect_area(rect));
    println!("Square Area: {}", rect_area(square(Point(5.0, 5.0), 1.0)));
}
