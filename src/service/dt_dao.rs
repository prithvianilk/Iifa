use std::fs::File;
use std::io::{Read, Write};
use crate::domain::node::Node;

// TODO: Remove when its a platform :)
const DT_FILE_PATH: &'static str = "dt.json";

#[derive(Clone, Copy)]
pub struct DtDao {}

impl DtDao {
    pub fn get_root(&self) -> Node {
        let mut contents = String::new();
        let mut file = File::open(DT_FILE_PATH).expect("Failed to open file");
        file.read_to_string(&mut contents).expect("Failed to read file");
        let value: Node = serde_json::from_str(&contents).expect("Failed to deserialize JSON");
        drop(file);
        value
    }

    pub fn save_dt(&self, root: &Node) {
        let json_string = serde_json::to_string(&root).expect("Failed to serialize struct to JSON");
        let mut file = File::create(DT_FILE_PATH).expect("Failed to create file");
        file.write_all(json_string.as_bytes()).expect("Failed to write to file");
    }
}