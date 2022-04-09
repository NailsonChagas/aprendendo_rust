struct Point<T, V>{ //T = any type
    x: T,
    y: T,
    z: V
}

fn generics(){
    let a = Point{x:1, y:1, z:true};
    println!("a: x = {} / y = {} / z = {}", a.x, a.y, a.z);
    let b = Point{x:1.2, y:1.2, z:1};
    println!("b: x = {} / y = {} / z = {}", b.x, b.y, b.z);
    let c = Point{x:true, y:false, z:1.8};
    println!("c: x = {} / y = {} / z = {}", c.x, c.y, c.z);
}

fn main() {
    generics();
}
