use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::domain::node::Node;

#[derive(Debug, Deserialize)]
pub struct CreateDecisionTreeRequest {
    pub root: Node,

    pub description: String,

    #[serde(default)]
    pub context: Value,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDecisionTreeRequest {
    pub _id: String,

    #[serde(default)]
    pub root: Node,

    pub description: String,

    #[serde(default)]
    pub context: Value,
}

#[derive(Debug, Deserialize)]
pub struct EvaluateRequest {
    pub input_params: Value
}

#[derive(Debug, Serialize)]
pub struct EvaluateResponse {
    pub result: String
}