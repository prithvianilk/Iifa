use std::fs::File;
use std::io::Write;
use crate::domain::node::Node;
use crate::util::file_util::read_file;

// TODO: Remove when its a platform :)
const DT_FILE_PATH: &'static str = "dt.json";

#[derive(Clone, Copy)]
pub struct DtDao {}

impl DtDao {
    pub fn get_root(&self) -> Node {
        read_file(&DT_FILE_PATH.to_string())
    }

    pub fn save_dt(&self, root: &Node) {
        let json_string = serde_json::to_string(&root).expect("Failed to serialize struct to JSON");
        let mut file = File::create(DT_FILE_PATH).expect("Failed to create file");
        file.write_all(json_string.as_bytes()).expect("Failed to write to file");
    }
}