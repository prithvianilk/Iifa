use serde::{Serialize, Deserialize};
use crate::domain::predicate::Predicate;

#[derive(Debug, Deserialize, Serialize)]
pub struct Graph {
    pub nodes: Vec<FlowNode>,
    pub edges: Vec<FlowEdge>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FlowNode {
    pub id: String,
    pub position: Position,
    pub data: Data,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Data {
    pub label: String,
    pub description: String,
    pub value: Option<String>,
    pub predicate: Predicate
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FlowEdge {
    pub id: String,
    pub source: String,
    pub target: String,
    pub label: Direction
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Direction {
   LEFT,
   RIGHT
}