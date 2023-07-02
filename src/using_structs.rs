
// use this in a main.rs file
use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Visitor {
        Visitor { 
            name: name.to_lowercase(), 
            greeting: greeting.to_string(), 
        }
    }
    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
    
}

fn user_name() -> String {
    println!("What is your name?");
    let mut your_name = String::new();
    stdin()
    .read_line(&mut your_name)
    .expect("Failed to read line...");
    your_name
    .trim()
    .to_lowercase()
    
    
}

fn main() {
    
let name = user_name();  
let visitors = 
[
    Visitor::new("Scotty", format!("welcome, {name}!").as_str()),
    Visitor::new("Lisa", format!("welcome, {name}!").as_str()),
    Visitor::new("Flora", format!("welcome, {name}!").as_str())
]; 
    
for visitor in 0..visitors.len() {
    if name == visitors[visitor].name{
        visitors[visitor].greet_visitor();
    }      
}

}