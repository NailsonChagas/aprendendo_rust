use std::mem;

fn array(){
    let a:[i32; 5] = [1,2,3,4,5];
    println!("TAMANHO {}", a.len());
    
    for i in 0..a.len(){
        println!("a[{}] = {}", i, a[i]);
    }

    let a:[i32; 11] = [1; 11];
    println!("TAMANHO {}", a.len());
    
    for i in 0..a.len(){
        println!("a[{}] = {}", i, a[i]);
    }

    println!("a usa {} bytes", mem::size_of_val(&a));

    let matriz: [[i32;2]; 3] = [[0,1],[1,0], [0,1]];
    for i in matriz{
        for j in i{
            print!("{} ", j);
        }
        print!("\n");
    }
    println!("{:?}", matriz); // :? debug
}

fn use_slice(slice: &mut [i32]){
    println!("first element = {}, len = {}", slice[0], slice.len());
    slice[0] = -1;
}

fn slices(){
    let mut data1 = [1,2,3,4,5];
    use_slice(&mut data1[1..4]);
    println!("{:?}", data1);

    let mut data2 = [1,2,3,4,5];
    use_slice(&mut data2);
    println!("{:?}", data2);
}

fn sum_product(x:i32, y:i32) -> (i32, i32){
    return (x+y, x*y);
}

fn tuples(){
    let x = 3;
    let y = 4;
    let sp = sum_product(x, y);
    println!("sp = {:?}", sp);                 //0  1    2    3
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    //destructioring
    let (a, b) = sp;
    println!("a = {}, b = {}", a, b);

    //tuplas de tuplas
    let sp2 = sum_product(4, 7);
    let combined = (sp,sp2);
    println!("{:?}", combined);
    println!("{}", (combined.1).0);
}

fn main() {
    println!("Hello, world!");
    array();
    slices();
    tuples();
}
