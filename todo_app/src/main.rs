mod to_do;
use to_do::structs::done::Done;
use to_do::structs::pending::Pending;

fn main() {
    let done = Done::new("Shopping");
    println!("{}", done.super_struct.title);
    println!("{}", done.super_struct.status.stringify());
    let pending = Pending::new("Cook dinner");
    println!("{}", pending.super_struct.title);
    println!("{}", pending.super_struct.status.stringify());
}
