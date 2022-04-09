fn say(frase: String){
    println!("{}", frase);
}

fn closure(){
    let fala = say;
    fala("Olá".into());

    //inline
    let plus_one = |x:i32| -> i32 {return x + 1;};
    
    let a = 6;
    let two:i32 = 2;

    {
        let plus_two = |x:i32| {
            let mut z = x;
            z += two;
            return z;
        };
        println!("{} + 2 = {}", a, plus_two(a));
    }
    
    println!("{} + 1 = {}", a, plus_one(a));

    // T: por cópia == passa uma cópia do valor para função
    // &T: por referencia == passa uma refencia apontando para o valor (valor n pode ser alterado)
    // &mut T por referencia mutavel == passa uma refencia apontando para o valor (valor pode ser alterado)
}

// higher-order functions
//função que pode receber função como parametro (callback)
//or
//função que pode retornar outra funçao (generator)
fn is_even(x: u32) -> bool {
    x % 2 == 0
}

fn greater_than(limit: u32) -> impl Fn(u32) -> bool {
    move |x| {return x > limit;}
}

pub fn demo() {
    // functions that take functions
    // functions that return functions

    // sum of all even squares <= 500

    let limit = 500;
    let mut sum = 0;

    //let above_limit_2 = |y| y > limit;
    let above_limit = greater_than(limit);

    for i in 0.. {
        let isq = i * i;

        //if isq > limit {
        if above_limit(isq) {
            break;
        } else if is_even(isq) {
            sum += isq;
        }
    }

    println!("loop sum = {}", sum);

    let sum2 = (0..)
        .map(|x| x * x)
        .take_while(|&x| x < limit)
        .filter(|x| is_even(*x))
        .fold(0, |sum, x| sum + x);
    println!("hof sum = {}", sum2);
}

fn main() {
    closure();
    demo();
}
