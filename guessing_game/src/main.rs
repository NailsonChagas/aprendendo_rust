use std::cmp::Ordering;
use colored::*;
use rand::Rng;
use std::io;

fn main() {
    let number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Entre sua escolha: ");
        let mut buffer = String::new();
        
        io::stdin().read_line(&mut buffer).expect("Falha ao ler entrada");
    
        let guess:u32 = match buffer.trim().parse::<u32>() {
            Ok(num) => {num},
            Err(_) => continue //próxima iteração
        };

        println!("Você escolheu {}", guess);

        match guess.cmp(&number) {
            Ordering::Less => {println!("{}", "Muito pequeno".red());}
            Ordering::Greater => {println!("{}", "Muito grande".red());}
            Ordering::Equal => {
                println!("{}", "Acertou !!!!".green());
                break;
            }
        }
    }
}