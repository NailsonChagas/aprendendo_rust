fn main() {
    /*
    String is the dynamic heap string type, like Vec: use it 
    when you need to own or modify your string data.

    str is an immutable1 sequence of UTF-8 bytes of dynamic 
    length somewhere in memory. Since the size is unknown, 
    one can only handle it behind a pointer. 
    This means that str most commonly2 appears as &str: 
    a reference to some UTF-8 data, normally called a 
    "string slice" or just a "slice". A slice is just a view 
    onto some data, and that data can be anywhere, e.g.
    */
    let name:&str = "Nailson"; //armazenado na stack
    let middle_name:String = String::from("Francisco"); //endereço armazenado na stack apontando para o valor na heap
    let greeting  = format!("Hi, I'm {} {}", name, middle_name);
    println!("{}", greeting);
    println!("Teste: {}", format!("{1}, {0}, {1}, {data}", name, middle_name, data = "Sei lá"))
}
