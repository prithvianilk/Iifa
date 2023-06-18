use std::{fs::File};
use std::io::Read;
use chrono::{Local, TimeZone};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
enum Predicate {
    AppliedBeforeDeadline{application_deadline: String},
    SalaryAbove{min_salary: i32},
    AgeAbove{min_age: i8},
    InGoodOccupations{occupations: Vec<String>},
    DEFAULT
}

impl Default for Predicate {
    fn default() -> Self { Predicate::DEFAULT }
}

#[derive(Debug, Deserialize, Serialize)]
struct CustomerParams {
    pub application_time: String,
    pub salary: i32,
    pub occupation: String,
    pub age: i8
}

fn is_date_before(x: &String, y: &String) -> bool {
    let fmt = "%Y-%m-%d %H:%M:%S";
    Local.datetime_from_str(&x, fmt).unwrap().timestamp_millis() < Local.datetime_from_str(&y, fmt).unwrap().timestamp_millis()
}

fn evaluate(predicate: &Predicate, customer_params: &CustomerParams) -> bool {
    match predicate {
        Predicate::AppliedBeforeDeadline{application_deadline} => is_date_before(&customer_params.application_time, application_deadline),
        Predicate::SalaryAbove { min_salary } => &customer_params.salary >= min_salary,
        Predicate::AgeAbove { min_age } =>  &customer_params.age >= min_age,
        Predicate::InGoodOccupations { occupations } => occupations.contains(&customer_params.occupation),
        Predicate::DEFAULT => panic!("Default predicate"),
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Node {
    #[serde(default)]
    pub description: String,

    #[serde(default)]
    pub predicate: Predicate,

    #[serde(default)]
    pub value: Option<String>,

    #[serde(default)]
    pub left: Option<Box<Node>>,

    #[serde(default)]
    pub right: Option<Box<Node>>,
}

impl Node {
    fn new(predicate: Predicate, description: String) -> Node {
        Self { description, predicate, value: None, left: None, right: None }
    }

    fn new_terminal(value: String) -> Node {
        Self { description: String::new(), predicate: Predicate::DEFAULT, value: Some(value), left: None, right: None }
    }

    fn link(&mut self, node: Node, is_left: bool) {
        if is_left {
            self.left = Some(Box::new(node))
        } else {
            self.right = Some(Box::new(node))
        }
    }

    fn traverse(&mut self, customer_params: &CustomerParams) -> Option<String> {
        let is_terminal = self.value.is_some();
        return if is_terminal {
            self.value.clone()
        } else if evaluate(&self.predicate, customer_params) {
            self.right.as_mut()
                .map(|c| c.traverse(customer_params))
                .unwrap_or(None)
        } else {
            self.left.as_mut()
                .map(|c| c.traverse(customer_params))
                .unwrap_or(None)
        };
    }
}

fn main() {
    let customer_params: CustomerParams = read_and_parse("customer_params.json");
    let mut node: Node = read_and_parse("dt.json");
    if let Some(value) = node.traverse(&customer_params) {
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