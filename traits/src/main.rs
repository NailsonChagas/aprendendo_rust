//relação entre structs

use std::fmt::Debug;

trait Animal {
    fn create(name: &'static str) -> Self;

    fn name(&self) -> &'static str; //deve ser implementado
    
    fn talk(&self){ //implementação default (caso n implementado essa sera a usada)
        println!("{} cannot talk", self.name());
    }
}

#[derive(Debug)]
struct Human{
    name: &'static str
}

impl Animal for Human {
    fn create(name: &'static str) -> Human{
        return Human{name: name};
    }

    fn name(&self) -> &'static str{
        return self.name;
    }
}

#[derive(Debug)]
struct Cat{
    name: &'static str
}

impl Animal for Cat {
    fn create(name: &'static str) -> Cat{
        return Cat{name: name};
    }

    fn name(&self) -> &'static str{
        return self.name;
    }

    fn talk(&self) {
        println!("{}: Miau", self.name());
    }
}

fn traits(){
    let h = Human::create("Nailson");
    h.talk();

    let cat = Cat::create("Frajola");
    cat.talk();

    print_talk(h);
    print_talk(cat);
}

fn print_talk<T:Animal + Debug>(animal: T){ //recebendo trait como parametro
    println!("{:?}", animal);
    animal.talk();
}

fn main() {
    traits();
}
