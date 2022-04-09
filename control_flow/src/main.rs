fn if_statement(temp:i8, drink:bool){
    if temp >= 30 {
        println!("Really hot outside");
        if drink == true {
            println!("Go get a cold coup of tea");
        }
    }
    else if temp <= 10 {
        println!("Really cold outside");
        if drink == true {
            println!("Go get a hot coup of tea");
        }
    }
    else {
        println!("Really good temperature outside");
        if drink == true {
            println!("Go buy a juice")
        }
    }

    let day = if (temp >= 30) & (drink == true) {"cold coup of tea"} else if (temp <= 10) & (drink == true) {"hot coup of tea"} else {"juice"};
    println!("Today is a grate day for a {}", day);
}

fn while_and_loop(inter:u8){
    let mut i = 1;

    while i < inter {
        i *= 2;
    }
    println!("while: i = {}", i);

    let mut i = 1;

    let a = loop {
        if i >= inter {
            break i;
        }
        i *= 2;
    };
    println!("loop retornando: a = {}", a);

    let mut i = 1;

    loop {
        if i >= inter {
            break;
        }
        i *= 2;
    };
    println!("loop não retornando: i = {}", i);
}

fn for_loop(max:u8){
    for x in 0..max{
        if x == 3{
            continue; //próxima iteração do loop
        }
        if x == 8{
            break; //sair do loop
        }
        println!("{x}");
    }

    for (pos, y) in (30..41).enumerate(){
        println!("Pos: {}, y = {}", pos, y);
    }
}

fn match_statement(number:u8){
    let number_str = match number {
        1 => {"um"},
        2 => {"dois"},
        3 => {"três"},
        4..=14 => {"entre 4 e 14"},
        _ => {"not today son -> invalid"} //caso n bata com nada
    };
    println!("O número é: {}", number_str);
}

fn main() {
    if_statement(20, true);
    while_and_loop(14);
    for_loop(12);
    match_statement(20);
}
