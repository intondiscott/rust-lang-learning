
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
    let greeting = format!("Welcome, {}!",name[..1].to_uppercase() + &name[1..]);
    let visitors = 
    [
        Visitor::new("Scotty", &greeting),
        Visitor::new("Lisa", &greeting),
        Visitor::new("Flora", &greeting)
    ]; 
    
    for visitor in 0..visitors.len() {
        if name == visitors[visitor].name{
            visitors[visitor].greet_visitor();
        }      
    }

}