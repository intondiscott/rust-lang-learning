struct User {
    name: String,
    age: u8,
    email: String,
}
impl User {
    fn greeting(&self) {
    println!("Welcome {}!",&self.name);
    }
    fn user_info(&self){
        println!("{{\n\tname: {}, \n\tage: {}, \n\temail: {}\n}}", &self.name, &self.age, &self.email);
    }
}

fn main() {
    let user1 = User{name: "Scotty".to_string(),age: 37, email: "intondiscott@gmail.com".to_string()};
    user1.greeting();
    user1.user_info();
}


