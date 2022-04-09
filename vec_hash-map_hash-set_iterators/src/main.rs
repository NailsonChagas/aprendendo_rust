use std::collections::HashMap;
use std::collections::HashSet;

fn iterators(){
    let vec1: Vec<u8> = vec![3,2,1];
    let mut vec2: Vec<u8> = vec![3,2,1];

    println!("vec1: {:?}", vec1);
    for x in vec1{ //move vec deixando ele inacessivel
        println!("{}", x);
    }
    println!("vazio");
    //println!("vec1: {:?}", vec1); //n possivel como explicado acima

    println!("vec2: {:?}", vec2);
    for x in &vec2{ //como esta passando por referencia pode ser utilizado depois 
        println!("{}", x);
    }
    println!("vec2: {:?}", vec2);

    for x in vec2.iter(){ //faz o msm que o acima praticamente
        println!("{}", x);
    }

    for x in vec2.iter().rev(){ //ordem reverso
        println!("{}", x);
    }

    println!("vec2: {:?}", vec2);
    for x in vec2.iter_mut(){ //faz o msm que o acima praticamente
        *x = *x * 2 + 1;
    }
    println!("vec2: {:?}", vec2);

    
}

fn hash_set(){ //valores unicos em um container
    let mut greeks:HashSet<&str> = HashSet::new();

    greeks.insert("gama"); //true
    greeks.insert("delta"); //true
    greeks.insert("alpha"); //true
    greeks.insert("vega"); //true
    greeks.insert("lambda"); //true
    greeks.insert("lambda"); //vai retornar falso pois o valor já existe
    println!("greeks: {:?}", greeks);

    if !greeks.contains("kappa"){
        println!("Não contem kappa");
    }
    if greeks.contains("alpha"){
        println!("Contem alpha");
    }

    let removed = greeks.remove("delta");
    if removed{
        println!("delta removido");
    }

    let _1_5: HashSet<i32> = (1..=5).collect();
    let _6_10: HashSet<i32> = (6..=10).collect();
    let _1_10: HashSet<i32> = (1..=10).collect();
    let _2_8: HashSet<i32> = (2..=8).collect();

    //subset
    println!("is {:?} subset de {:?}? {}", _2_8, _1_10, _2_8.is_subset(&_1_10));

    //disjoint = sem elementos em comum
    println!("is {:?} disjoint de {:?}? {}", _1_5, _6_10, _1_5.is_disjoint(&_6_10));

    //unium, intersection --> retorna um hash_set com todos valores unicos dos dois
    println!("is {:?} subset de {:?}? {:?}", _2_8, _1_10, _2_8.union(&_1_10));

    //difference = 
    //symmetrinc_difference = union - intersection
}

fn hash_map(){ //parecido com um dict em python
    let mut shapes: HashMap<String, u8> = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    println!("shapes: {:?}", shapes);

    println!("triangle has {} sides", shapes["triangle"]);
    println!("square has {} sides", shapes["square"]);

    for (key, value) in &shapes{
        println!("{} : {}", key, value);
    }

    shapes.insert("square".into(), 5);

    for (key, value) in &shapes{
        println!("{} : {}", key, value);
    }

    shapes.entry("circle".into()).or_insert(1); //se não existir insere com o valor 1
    for (key, value) in &shapes{
        println!("{} : {}", key, value);
    }

    {
        //referencia para circle
        let actual = shapes.entry("circle".into()).or_insert(2);
        *actual = 0; //muda o valor de circle
    }

    for (key, value) in &shapes{
        println!("{} : {}", key, value);
    }
}

fn vectors(){
    let mut a: Vec<i8> = Vec::new(); //vector -> armazena na heap / array -> armazena no stack
    let idx:usize = 0; //indices devem ser unsigned com numero de bits coerente ao processador

    println!("a = {:?}", a);
    for i in 0..17{
        a.push(i);
        println!("a = {:?}", a);
    }

    println!("--------------------------------");
    match a.get(6) {
        Some(x) => println!("a[6] = {}", x),
        None => println!("Error, elemento não existe")
    }
    println!("{}", &a[idx]);
    for x in &a {println!("{}", x)}
    println!("--------------------------------");

    for _i in 0..a.len(){
        a.pop();
        println!("a = {:?}", a);
    }

    for i in 0..17{
        a.push(i);
    }
    while let Some(x) = a.pop() { //enquanto elemento retirado exista
        println!("{}", x);
    }
    
}
fn main() {
    vectors();
    hash_map();
    hash_set();
    iterators();
}
