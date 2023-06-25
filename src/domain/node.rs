use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::domain::predicate::{Predicate, evaluate};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Node {
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
    pub fn new(predicate: Predicate, description: String) -> Node {
        Self { description, predicate, value: None, left: None, right: None }
    }

    pub fn new_terminal(value: String) -> Node {
        Self { description: String::new(), predicate: Predicate::Default, value: Some(value), left: None, right: None }
    }

    pub fn link(&mut self, node: Node, is_left: bool) {
        if is_left {
            self.left = Some(Box::new(node.clone()))
        } else {
            self.right = Some(Box::new(node.clone()))
        }
    }

    pub fn traverse(&mut self, customer_params: &Value, context: &Value) -> Option<String> {
        let is_terminal = self.value.is_some();
        return if is_terminal {
            self.value.clone()
        } else if evaluate(&self.predicate, customer_params, context) {
            self.right.as_mut()
                .map(|c| c.traverse(customer_params, context))
                .unwrap_or(None)
        } else {
            self.left.as_mut()
                .map(|c| c.traverse(customer_params, context))
                .unwrap_or(None)
        };
    }
}