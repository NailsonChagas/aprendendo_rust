fn main() {
    //variaveis
    let x = 5; //imutavel
    let mut y = 5; //mutavel == variavel
    const PI: f32 = 3.1415; //constantes não podem ser mutaveis, e devem ter tipo declarado

    //tipos de de dados
    //scaler datatipes
    /*Integer: 
        -signed: i8, i16, i32, i64, i128, isize
        -unsigned: u8, u16, u32, u64, u128, usize
    */

    //compound types
    let tupla = ("teste", 29);
    let copia = tupla;
    let teste_str = tupla.0;

    let arrray = [200, 400, 800];
    let item = arrray[0];

    //funcões
    let te = my_function(arrray);

    //controle de fluxo
    if te < 10{
        println!("a");
    }
    else if te > 90 {
        println!("b");
    }
    else{
        println!("c");
    }

    let numero = if te > 5 {10} else {1};

    let mut i = 0;
    let resultado = loop {
        println!("Loop");
        i += 1;
        if i == 10 {
            break i; //retorna i
        }
    };

    while i != 0 {
        i -= 1;
    }

    for element in arrray.iter(){
        println!("{}", element);
    }

    for element in 1..8{
        println!("{}", element);
    }
}

fn my_function(x:[i32; 3]) -> i32{ //recebe um array de tamanho 3 e retorna um int de 32bit
    print!("Função ");
    let sum = x[0] + x[1] + x[2];
    return sum;
}