union IntOrFloat{
    i: i32,
    f: f32
}

fn process_value(iof: IntOrFloat){
    unsafe{ //unsafe pq n ha como saber se o que esta dentro é int ou float
        match iof {
            IntOrFloat{i} => { //vai tentar imprimir como int
                println!("integer, value = {}", iof.i);
            },
        }
    }
}

fn main() {
    let oof = IntOrFloat{i:123};
    process_value(oof);

    let oof = IntOrFloat{f:123.0};
    process_value(oof);
}
