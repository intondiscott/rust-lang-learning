
//put in main.rs file to run
use std::io::stdin;


#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote {note: String},
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: u8,
}
impl Visitor {
    fn new(name: &str, action: VisitorAction, age:u8) -> Visitor {
        Visitor { 
            name: name.to_lowercase(), 
            action, 
            age,
        }
    }
    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome {}", self.name),
            VisitorAction::AcceptWithNote { note } => println!("Welcome {}, {}", self.name, note),
            VisitorAction::Probation => {
                println!("{} is on the probation period.", self.name);
                if self.age < 21 {
                    println!("{} can't drink until 21 years of age which is {} yrs from now.", self.name, 21 - self.age)
                }
        }
            VisitorAction::Refuse => println!("Do not let {} in.", self.name)
        }
    }
}


fn main() {
    let mut visitor_list = vec![
        Visitor::new("Scotty", VisitorAction::AcceptWithNote { note: "how is your stay?".to_string() }, 37),
        Visitor::new("Lisa",VisitorAction::Probation, 17),
    ];
    visitor_list.push(Visitor::new("John", VisitorAction::Accept, 21));
            
    for visitor in visitor_list {
        visitor.greet_visitor()
    }
}