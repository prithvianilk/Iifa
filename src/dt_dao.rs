use std::{fs::File};
use std::io::{Read, Write};
use serde::de::DeserializeOwned;
use crate::node::Node;

pub struct DtDao {
    file_path: String
}

impl DtDao {
    pub fn new(file_path: &str) -> DtDao {
        Self { file_path: file_path.to_string() }
    }

    pub fn get_root(&self) -> Node {
        read_and_parse(&self.file_path)
    }

    pub fn save_dt(&self, root: Node) {
        let json_string = serde_json::to_string(&root).expect("Failed to serialize struct to JSON");
        let mut file = File::create(&self.file_path).expect("Failed to create file");
        file.write_all(json_string.as_bytes()).expect("Failed to write to file");
    }
}

fn read_and_parse<T>(path: &str) -> T where T: DeserializeOwned {
    let mut contents = String::new();
    let mut file = File::open(path).expect("Failed to open file");
    file.read_to_string(&mut contents).expect("Failed to read file");
    let value: T = serde_json::from_str(&contents).expect("Failed to deserialize JSON");
    drop(file);
    value
}