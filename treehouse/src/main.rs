use std::{io::{stdin}, thread::AccessError};

#[derive(Debug)]
struct Visitor {
    name: String,
    action: Action,
    age: i8,
}


// Debug allows you to print enum values by name
#[derive(Debug)]
enum Action {
    Accept,
    AcceptWithNote {
        note: String
    },
    Refuse,
    Probation
}

impl Visitor {
    // new() is the equivalent of a constructor
    // new() is also a function associated w/ the Visitor type
    // Notice the lack of a &self as the first param.
    // The return value Self is the type of the current object
    // To call this function, you need to do Visitor::new()
    // You can think of this as a class method whereas 
    // greet_visitor() is an instance method.
    fn new(name :&str, action: Action, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),      // to_lowercase() returns a String!
            action,
            age
        }
    }
    // This takes a reference of self because it doesn't want
    // to take ownership of self.
    fn greet_visitor(&self) {
        match &self.action {
            Action::Accept => println!{"Welcome to the Shadowy Super Coder DAO {}", self.name},
            Action::AcceptWithNote {note} => {
                println!("Welcome to the treehouse {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            },
            Action::Probation => println!{"{} is now a probationary member", self.name},
            Action::Refuse => println!{"No entry for you!"},
        }
    }
}

fn main() {
    let mut visitors = vec![
        Visitor::new("bert", Action::Accept, 45),
        Visitor::new("sam", Action::AcceptWithNote{note: String::from("lactose intolerant")}, 15),
        Visitor::new("Simon", Action::Refuse, 30)
    ];
    loop {
        println!("\nHello, what's your name? (leave empty and press enter to quit)");
        let name = ask_name();
        // Turns an array into an iterator
        // use the .find() function to find the Option<visitor>
        // within this array that matches the condition
        // v.name == name
        // the |v| v.name == v is a closure but you can
        // think of it as a function for now.
        let guest = visitors.iter().find(|v| v.name == name);
        match guest {
            Some(v) => v.greet_visitor(),
            None => {
                // Give users a way to exit the program
                // don't forget to prompt the user in the entry text!
                // is_empty() is more efficient then name.len() == 0
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is now a fren, try entering again!", name);
                    visitors.push(Visitor::new(&name, Action::Probation, 0));
                }
            }
        }
    }
    
    
}

fn ask_name() -> String {
    let mut name = String::new();
    stdin().read_line(&mut name).expect("Failed to read line");
    // When you print w/ debug a string, sometimes you might find things
    // like \r (carriage return) or \n (new line)
    // You should use trim() to remove these and to_lowercase() so that
    // all the stdin you read are all of the same case.
    // For example, if you removed the .trim(), you'll realise that the
    // exclamation mark always appears on the next line
    name.trim().to_lowercase()
}
