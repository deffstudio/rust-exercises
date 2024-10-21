use std::{
    collections::HashSet,
    io::{self, Write},
};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Item {
    Key,
    Book,
    Candle,
    Map,
}
enum Room {
    Entrance,
    Hallway,
    Library,
    Treasure,
    Study,
}

struct GameState {
    current_room: Room,
    inventory: HashSet<Item>,
}

fn main() {
    let mut game = GameState {
        current_room: Room::Entrance,
        inventory: HashSet::new(),
    };

    println!("Welcome to the Enhanced Rust Text Adventure!");
    println!("You find yourself at the entrance of a mysterious mansion.");

    loop {
        match game.current_room {
            Room::Entrance => entrance_room(&mut game),
            Room::Hallway => hallway_room(&mut game),
            Room::Library => library_room(&mut game),
            Room::Treasure => treasure_room(&mut game),
            Room::Study => study_room(&mut game),
        }
    }
}

fn entrance_room(game: &mut GameState) {
    println!("\nYou are in the entrance room. There's a door leading to a hallway.");
    println!("What would you like to do?");
    println!("1. Enter the hallway");
    println!("2. Check inventory");
    println!("3. Quit the game");

    match get_input().as_str() {
        "1" => game.current_room = Room::Hallway,
        "2" => display_inventory(game),
        "3" => {
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
    println!("2. Enter the study");
    println!("3. Enter to the treasure room");
    println!("4. Go back to the entrance");
    println!("5. Check inventory");

    match get_input().as_str() {
        "1" => game.current_room = Room::Library,
        "2" => game.current_room = Room::Study,
        "3" => {
            if game.inventory.contains(&Item::Key) {
                game.current_room = Room::Treasure;
            } else {
                println!("The door is locked. You need a key to enter.");
            }
        }
        "4" => game.current_room = Room::Entrance,
        "5" => display_inventory(game),
        _ => println!("Invalid choice. Please try again."),
    }
}

fn library_room(game: &mut GameState) {
    println!("\nYou are in the library. There's a dusty old book on a shelf.");
    println!("What would you like to do?");
    println!("1. Examine the book");
    println!("2. Take the candle");
    println!("3. Go back to the hallway");
    println!("4. Check inventory");

    match get_input().as_str() {
        "1" => {
            if !game.inventory.contains(&Item::Book) {
                println!("You found a key hidden in the book!");
                game.inventory.insert(Item::Book);
            } else {
                println!("You've already taken then book.");
            }
        }
        "2" => {
            if !game.inventory.contains(&Item::Candle) {
                println!("You take the candle. It might come in handy in dark places.");
                game.inventory.insert(Item::Candle);
            } else {
                println!("You've already taken the candle.");
            }
        }
        "3" => game.current_room = Room::Hallway,
        "4" => display_inventory(game),
        _ => println!("Invalid choice. Try again."),
    }
}

fn study_room(game: &mut GameState) {
    println!(
        "\nYou enter a cozy study room. There's a desk with a locked drawer and a map on the wall."
    );
    println!("What would you like to do?");
    println!("1. Examine the desk");
    println!("2. Take the map");
    println!("3. Go back to the hallway");
    println!("4. Check inventory");

    match get_input().as_str() {
        "1" => {
            if game.inventory.contains(&Item::Key) {
                println!("You use the key to unlock the drawer. Inside, you find a note:");
                println!("'The treasure awaits those who light the way.");
            } else {
                println!("The drawer is locked. You need a key to open it.");
            }
        }
        "2" => {
            if !game.inventory.contains(&Item::Map) {
                println!("You take the map from the wall. It shows the layout of the mansion.");
                game.inventory.insert(Item::Map);
            } else {
                println!("You've already taken the map.");
            }
        }
        "3" => game.current_room = Room::Hallway,
        "4" => display_inventory(game),
        _ => println!("Invalid choice. Please try again."),
    }
}

fn treasure_room(game: &mut GameState) {
    println!("\nYou've entered the treasure room!");
    if game.inventory.contains(&Item::Candle) {
        println!("The room is dark, but you light your candle. In the flickering light, you see a massive treasure chest!");
        println!("Congratulations! You've won the game!");
    } else {
        println!("The room is pitch black. You can't see anyting. Maybe you need a light source?");
        println!("You decide to return to the hallway for now.");
        game.current_room = Room::Hallway;
    }
}

fn display_inventory(game: &GameState) {
    if game.inventory.is_empty() {
        println!("Your inventory is empty.");
    } else {
        println!("Your inventory contains:");
        for item in &game.inventory {
            println!("- {:?}", item);
        }
    }
}

fn get_input() -> String {
    println!("> ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_lowercase()
}
