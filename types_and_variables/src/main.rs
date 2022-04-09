/*------- Tipos e Variaveis -------
Obs:
    * bit e byte
        - bit = 0 ou 1 (menor tamanho que uma informação pode ter, 2¹ valores possiveis)
        - byte = conjunto de 8 bits (2⁸ valores possiveis)
        - 2 bytes = 2^(16)
        - 4 bytes = 2^(32)
        - 8 bytes = 2^(64)
    
    * unsigned e signed (exemplo usando int)
        - unsigned (números não negativos)
            + 8 bits -> [0, 255]
            + 16 bits -> [0, 65535]
            + 32 bits -> [0, 4294967295]
            + 64 bits -> [0, 18446744073709551615]
        - signed (números possivelmente negativos)
            + 8 bits -> [-128, 127]
            + 16 bits -> [-32768, 32767]
            + 32 bits -> [-2147483648, 2147483647]
            + 64 bits -> [-9223372036854775808, 9223372036854775807]

    * floating-point numbers
        - 32 bits -> float (single precision)
        - 64 bits -> double (double precision)
        - floating-point não representão exatamente os numeros:
            + Exemplo 0.1 + 0.2 não resultara exatamente em 0.3
        - podem respresentar valores especiais
            + mais e menos infinito
            + NaN
    
immutable = valor não pode ser alterado após ser declarado
mutable = pode ser alterado após a declaração
*/

mod stack_heap;
use std::mem;

const MEANING_OF_LIFE:u8 = 42; // constante --> não mutavel --> diferente de variaveis não possui endereço definido 
static Z:i32 = 123; // variavel estatica --> possui endereço definido -> pode ser mutavel porem deve se usar o unsafe 
static mut A:i32 = 1;

fn scope_and_shadowing(){
    let mut a = 123;
    println!("Fora do escopo: a = {}", a);

    {//escopo
        a += 70; //mesma variavel que a de fora
        let b = 456;
        println!("Dentro do escopo: b = {}", b);
        println!("Dentro do escopo: a = a + 70 = {}", a);
        let a = 77; //a declarado aqui n é o mesmo do de fora
        println!("Dentro do escopo: a = {}", a);
    }

    println!("Fora do escopo: a = {}", a);
    //println!("{}", b); --> fora do escopo, n funciona
}

fn operators(){
    let mut a = 2 + 3 * 4; // + - * /
    println!("{}", a);
    a = a + 1; // not support ++ --
    a += 1;
    a -= 1;
    a = a % 2; // resto de divisão
    a %= 2;
    println!("{}", a);

    a = 4;
    a = i32::pow(a, 3); //potencia
    println!("{}", a);

    let b = 2.5;
    let b_cubo = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("b_cubo = {}, usa {} byte(s) de memória", b_cubo, mem::size_of_val(&b_cubo));
    println!("b_to_pi = {}, usa {} byte(s) de memória", b_to_pi, mem::size_of_val(&b_to_pi));

    //operações com bit
    let c = 1 | 2; // | = OR / & = AND / ^ = XOR / ! = NOR
    println!("1 | 2 = {}", c); // 1 = 01 / 2 = 10 / 01 or 10 == 11 = 3
    let dois_elevado_a_10 = 1 << 10; //Bitwise shift operator - left
    println!("{}", dois_elevado_a_10);

    //operações logicas
    let pi_menor_que_4 = std::f64::consts::PI < 4.0;
    println!("{}", pi_menor_que_4); //true
}

fn main() {
    //println é um macro não uma função

    let a: u8 = 123; // immutable unsigned integer 8 bits
    //a = 14 -> não funciona, a é imutavel
    let mut b: i8 = 0; // mutable signed integer 8bits
    println!("a = {}\nb = {} antes", a, b);
    b = 14;
    println!("a = {}\nb = {} depois", a, b);
    println!("b = {}, usa {} byte(s) de memória", b, mem::size_of_val(&b));

    let a: f32 = 9.3;
    let b: f64 = 9.3;
    println!("a = {}, usa {} byte(s) de memória", a, mem::size_of_val(&a));
    println!("b = {}, usa {} byte(s) de memória", b, mem::size_of_val(&b));

    let a: bool = true;
    let b: bool = false;
    println!("a = {}, usa {} byte(s) de memória", a, mem::size_of_val(&a));
    println!("b = {}, usa {} byte(s) de memória", b, mem::size_of_val(&b));

    operators();
    scope_and_shadowing();

    println!("{}", MEANING_OF_LIFE);
    println!("{}", Z);

    unsafe {
        A += 1;
        println!("{}", A);
    }

    stack_heap::stack_heap();
}
