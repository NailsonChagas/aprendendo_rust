fn main() {
    let x = 3.0;
    let y = 1.0;

    // Option -> Some(v) | None
    let result = if y != 0.0 {Some(x/y)} else {None};
    
    match result {
        Some(z) => println!("{}/{}={}", x, y, z),
        None => println!("cannot divide by 0")
    }

    if let Some(z) = result {
        println!("result = {}", z);
    }
}
