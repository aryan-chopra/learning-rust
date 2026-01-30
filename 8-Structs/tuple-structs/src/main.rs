#[allow(dead_code)]

#[derive(Debug)]
struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    let Point(x, y, z) = origin;
    
    println!("{:#?}", black);
    println!("{x}, {y}, {z}");
}
