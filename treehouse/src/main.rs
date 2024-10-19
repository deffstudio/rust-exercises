use std::io::stdin;

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the treehouse!, {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the treehouse!, {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            }
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name),
            VisitorAction::Probation => {
                println!("{} is now probation member!", self.name)
            }
        }
    }
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("Steve", VisitorAction::Accept, 45),
        Visitor::new(
            "Sarah",
            VisitorAction::AcceptWithNote {
                note: String::from("Lactose-free milk is in the fridge"),
            },
            14,
        ),
        Visitor::new("Budi", VisitorAction::Probation, 21),
        Visitor::new("Rina", VisitorAction::Refuse, 17),
    ];

    loop {
        println!("What's your name?");
        let name = what_is_your_name();
        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not on the visitor list!", name);
                    visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 0));
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
