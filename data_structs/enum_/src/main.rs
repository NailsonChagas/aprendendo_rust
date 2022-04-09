enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8), //tuple
    CmykColor{ //struct
        cyan:u8,
        magenta:u8,
        yellow: u8,
        black: u8
    }
}

fn enums(c: Color){
    match c {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue"),
        Color::RgbColor(0, 0, 0) => println!("black"),
        Color::RgbColor(r, g, b) => println!("rgb({}, {}, {})", r, g, b),
        Color::CmykColor{ cyan:1, magenta:1, yellow:1, black:255 } => println!("n sei"),
        Color::CmykColor{ cyan:_, magenta:_, yellow:_, black:255 } => println!("Black"),
        Color::CmykColor{ cyan:_, magenta:_, yellow:_, black:_ } => println!("Qualquer cor")
    }
}

fn main() {
    let c = Color::Red;
    enums(c);
    enums(Color::Green);
    enums(Color::Blue);
    enums(Color::RgbColor(0, 0, 0));
    enums(Color::RgbColor(255, 10, 90));
    enums(Color::CmykColor{cyan:1, magenta:1, yellow:1, black:255});
}
