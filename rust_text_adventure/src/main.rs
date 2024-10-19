use std::io::{self, Write};

enum Room {
    Entrance,
    Hallway,
    Library,
    Treasure,
}

struct GameState {
    current_room: Room,
    has_key: bool,
}

fn main() {
    let mut game = GameState {
        current_room: Room::Entrance,
        has_key: false,
    };

    println!("Welcome to the Rust Text Adventure!");
    println!("You find yourself at the entrance of a mysterious mansion.");

    loop {
        match game.current_room {
            Room::Entrance => entrance_room(&mut game),
            Room::Hallway => hallway_room(&mut game),
            Room::Library => library_room(&mut game),
            Room::Treasure => treasure_room(&mut game),
        }
    }
}

fn entrance_room(game: &mut GameState) {
    println!("\nYou are in the entrance room. There's a door leading to a hallway.");
    println!("What would you like to do?");
    println!("1. Enter the hallway");
    println!("2. Quit the game");

    match get_input().as_str() {
        "1" => game.current_room = Room::Hallway,
        "2" => {
            println!("Thanks for playing!");
            std::process::exit(0);
        }
        _ => println!("Invalid option. Please try again."),
    }
}

fn hallway_room(game: &mut GameState) {
    println!("\nYou are in a long hallway. There are doors to a library and a tresure room.");
    println!("What would you like to do?");
    println!("1. Enter to the library");
    println!("2. Enter to the treasure room");
    println!("3. Go back to the entrance");

    match get_input().as_str() {
        "1" => game.current_room = Room::Library,
        "2" => {
            if game.has_key {
                game.current_room = Room::Treasure;
            } else {
                println!("The door is locked. You need a key to enter.");
            }
        }
        "3" => game.current_room = Room::Entrance,
        _ => println!("Invalid option. Please try again."),
    }
}

fn library_room(game: &mut GameState) {
    println!("\nYou are in the library. There's a dusty old book on a shelf.");
    println!("What would you like to do?");
    println!("1. Examine the book");
    println!("2. Go back to the hallway");

    match get_input().as_str() {
        "1" => {
            if !game.has_key {
                println!("You found a key hidden in the book!");
                game.has_key = true;
            } else {
                println!("You've already found the key in this book.");
            }
        }
        "2" => game.current_room = Room::Hallway,
        _ => println!("Invalid option. Please try again."),
    }
}

fn treasure_room(_game: &mut GameState) {
    println!("\nCongratulations! You've entered the treasure room and won the game");
    println!("Thanks for playing!");
    std::process::exit(0);
}

fn get_input() -> String {
    println!("> ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_lowercase()
}
