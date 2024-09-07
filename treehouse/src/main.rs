use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_lowercase(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("john", "hello"),
        Visitor::new("jane", "hi"),
        Visitor::new("bill", "hey"),
        Visitor::new("sarah", "salut"),
        Visitor::new("tim", "greetings"),
        Visitor::new("emily", "hola"),
        Visitor::new("oliver", "bonjour"),
        Visitor::new("james", "privet"),
        Visitor::new("lucy", "ciao"),
        Visitor::new("mike", "hallo"),
    ];

    loop {
        println!("What's your name?");
        let name = what_is_your_name();
        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

        match known_visitor {
            Some(visitor) => {
                println!("{}, {}!", visitor.greeting, name);
                visitor.greet_visitor();
            }
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not on the visitor list!", name);
                    visitor_list.push(Visitor::new(&name, "New Friend"));
                }
            }
        }
    }
    println!("The final list of visitors:");
    println!("{:#?}", visitor_list);
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}
