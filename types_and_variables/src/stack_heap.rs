use std::mem;
struct Point{
    x: f64,
    y: f64
}

fn origin() -> Point {
    let x = Point{x: 0.0, y: 0.0};
    println!("X:{} Y:{}", x.x, x.y);
    return x;
}

pub fn stack_heap(){
    let p1 = origin();
    let p2 = Box::new(origin()); //p2 é o endereço da localização da struct Ponto 

    println!("p1 ocupa {} bytes na stack", mem::size_of_val(&p1));
    let p3 = *p2; //p3 recebe ponto em p2
    println!("p3 ocupa {} bytes na stack", mem::size_of_val(&p3));
}