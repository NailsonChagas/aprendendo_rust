struct Point{
    x: f64,
    y: f64
}

struct Line{
    start: Point,
    end: Point
}

fn structures(){
    let p1 = Point{x:2.0, y:9.0};
    let p2 = Point{x:2.0, y:9.0};
    println!("p1: x = {} / y = {}", p1.x, p1.y);
    let linha = Line{start:p1, end:p2};   
}

fn main() {
    structures();
}
