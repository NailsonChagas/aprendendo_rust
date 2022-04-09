use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Adivinhe o número:");

    let secret = rand::thread_rng().gen_range(1, 101);
    println!("{}", secret);

    loop {
        println!("Insira sua escolha.");
        let mut guess = String::new(); //por padrão é uma constante, para variavel colovar mut
        
        //entrada de dados ---> &mut guess referencia mutavel para guess
        io::stdin().read_line(&mut guess).expect("Falha ao ler a linha");
        
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue //próxima iteração do loop
        };

        println!("Você escolheu: {}", guess);

        match guess.cmp(&secret){
            Ordering::Less => println!("{}", "Muito pequeno".red()),
            Ordering::Greater => println!("{}", "Muito grande".red()),
            Ordering::Equal => {
                println!("{}", "Você venceu".green());
                break;
            }
        }
    }
}
