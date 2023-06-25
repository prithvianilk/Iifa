use std::{fs::File, io::Read};

use serde::de::DeserializeOwned;

pub fn read_file<T>(path: &String) -> T where T: DeserializeOwned {
    let mut contents = String::new();
    let mut file = File::open(path).expect("Failed to open file");
    file.read_to_string(&mut contents).expect("Failed to read file");
    let value: T = serde_json::from_str(&contents).expect("Failed to deserialize JSON");
    drop(file);
    value
}