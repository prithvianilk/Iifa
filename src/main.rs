use std::{fs::File};
use std::io::Read;
use serde::de::DeserializeOwned;

mod node;
use node::Node;

mod customer_params;
use customer_params::CustomerParams;

mod predicate;

fn main() {
    let customer_params: CustomerParams = read_and_parse("customer_params.json");
    let mut root: Node = read_and_parse("dt.json");
    if let Some(value) = root.traverse(&customer_params) {
        println!("{}",value);
    } 
}

fn read_and_parse<T>(path: &str) -> T where T: DeserializeOwned {
    let mut file = File::open(path).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    let value: T = serde_json::from_str(&contents).expect("Failed to deserialize JSON");
    value
}