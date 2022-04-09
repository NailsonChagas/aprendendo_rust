struct User {
    username: String,
    email: String,
    sing_in_count: u64,
    active: bool
}

impl User {
    fn build_user(email:String, username:String) -> User{
        let new = User {
            email,
            username,
            active: true,
            sing_in_count: 1
        };
        return new;
    }

    fn imprimir(&self){
        println!("{}", self.sing_in_count);
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("sad@sdad,com"),
        username: String::from("dsfa"),
        active: true,
        sing_in_count: 1
    };

    //let name = user1.username;
    user1.imprimir();
    let name2 = User::build_user(String::from("sad"), String::from("sad"));
}
