fn how_many(x:i32) -> &'static str{
    match x {
        0 => "no",
        1 | 2 => "one or two",
        9..=11 => "lots of oranges",
        12 => "a dozen",
        _ if (x % 2 == 0) => "some",
        _ => "a few"
    }
}

fn pattern_matching(){
    for x in 0..13{
        println!("{}: I have {} oranges", x, how_many(x));
    }
}

fn main() {
    pattern_matching()
}
