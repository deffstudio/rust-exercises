mod state;
mod to_do;

use to_do::enums::TaskStatus;
use to_do::to_do_factory;
use to_do::ItemTypes;

use serde_json::value::Value;
use serde_json::{json, Map};
use state::{read_file, write_file};
use std::env;

use crate::to_do::traits::delete::Delete;
use crate::to_do::traits::edit::Edit;
use crate::to_do::traits::get::Get;

fn main() {
    let args: Vec<String> = env::args().collect();
    let status: &String = &args[1];
    let title: &String = &args[2];
    let mut state: Map<String, Value> = read_file("./state.json");
    println!("Before operation: {:?}", state);
    state.insert(title.to_string(), json!(status));
    println!("After operation: {:?}", state);
    write_file("./state.json", &mut state);
}
