struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

impl Line {
    fn new(start: Point, end: Point) -> Line{
        let new: Line = Line {
            start: start,
            end: end
        };
        return new;
    }

    fn len(&self) -> f64{
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;

        return f64::sqrt((f64::powi(dx, 2)) + (f64::powi(dy, 2)));
    }
}

fn main() {
    let linha = Line::new(Point{x:3.0, y:4.0}, Point{x:5.0, y:10.0});
    println!("Tamnho da linha: {:.2}", linha.len());
}
