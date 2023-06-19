use actix_web::{web, Responder, HttpResponse, get, put, post};
use serde::{Serialize, Deserialize};

use crate::{dt_dao::{DtDao}, node::{Node}, customer_params::CustomerParams, predicate::Predicate};

const STEP: f64 = 200.0;

#[get("/react-flow/dt")]
pub async fn get_as_flow() -> impl Responder {
    let dao = DtDao::new("dt.json");
    let root = dao.get_root();
    let mut nodes: Vec<FlowNode> = Vec::new();
    let mut edges: Vec<FlowEdge>= Vec::new();
    populate_nodes_and_edges(&root, 1, &Position { x: 0.0, y: 0.0 }, &mut nodes, &mut edges);
    HttpResponse::Ok().json(Graph{nodes, edges})
}

fn populate_nodes_and_edges(root: &Node, id: u32, position: &Position, nodes: &mut Vec<FlowNode>, edges: &mut Vec<FlowEdge>) {
    let is_terminal = root.value.is_some();
    if is_terminal {
        nodes.push(FlowNode{id: id.to_string(), position: position.clone(), data: Data { label: root.value.clone().unwrap(), description: root.description.clone(), value: root.value.clone(), predicate: root.predicate.clone()  }});
        return
    }
    nodes.push(FlowNode{id: id.to_string(), position: position.clone(), data: Data { label: root.description.clone(), description: root.description.clone(), value: root.value.clone(), predicate: root.predicate.clone()  }});
    if let Some(left) = &root.left {
        let edge_id = format!("{}-{}", id, id * 2);
        edges.push(FlowEdge { id: edge_id, source: id.to_string(), target: (id * 2).to_string(), label: Direction::LEFT });
        populate_nodes_and_edges(left, id * 2, &Position { x: position.x - STEP, y: position.y + STEP }, nodes, edges)
    }
    if let Some(right) = &root.right {
        let edge_id = format!("{}-{}", id, id * 2 + 1);
        edges.push(FlowEdge { id: edge_id, source: id.to_string(), target: (id * 2 + 1).to_string(), label: Direction::RIGHT });
        populate_nodes_and_edges(right, id * 2 + 1, &Position { x: position.x + STEP, y: position.y + STEP }, nodes, edges)
    }
}

#[put("/react-flow/dt")]
pub async fn save_from_flow(request: web::Json<Graph>) -> impl Responder {
    let dao = DtDao::new("dt.json");
    let graph = request.0;
    let root = construct_root(&graph);
    dao.save_dt(*root.unwrap());
    HttpResponse::Ok().json("{}")
}

fn construct_root(graph: &Graph) -> Option<Box<Node>> {
    let nodes = &graph.nodes;
    let edges = &graph.edges;

    let root_node = nodes.iter().find(|node| !has_incoming_edge(node, edges));
    if let Some(root_node) = root_node {
        Some(construct_subtree(root_node, nodes, edges))
    } else {
        None
    }
}

fn construct_subtree(node: &FlowNode, nodes: &[FlowNode], edges: &[FlowEdge]) -> Box<Node> {
    let mut root = Node {
        description: node.data.description.clone(),
        predicate: node.data.predicate.clone(),
        value: node.data.value.clone(),
        left: None,
        right: None,
    };

    let outgoing_edges = edges.iter().filter(|edge| edge.source == node.id);
    for edge in outgoing_edges {
        if let Some(target_node) = nodes.iter().find(|node| node.id == edge.target) {
            let child_node = construct_subtree(target_node, nodes, edges);
            root.link(*child_node, edge.label == Direction::LEFT);
        }
    }

    Box::new(root)
}

fn has_incoming_edge(node: &FlowNode, edges: &[FlowEdge]) -> bool {
    edges.iter().any(|edge| edge.target == node.id)
}

#[get("/dt")]
pub async fn get() -> impl Responder {
    let dao = DtDao::new("dt.json");
    let root = dao.get_root();
    HttpResponse::Ok().json(root)
}

#[put("/dt")]
pub async fn save(request: web::Json<Node>) -> impl Responder {
    DtDao::new("dt.json").save_dt(request.0);
    HttpResponse::Ok()
}

#[post("/dt/evaluate")]
pub async fn evaluate(request: web::Json<CustomerParams>) -> impl Responder {
    let dao = DtDao::new("dt.json");
    let result = dao.get_root().traverse(&request.0);
    return if result.is_some() {
        HttpResponse::Ok().json(EvaluateResponse{result: result.unwrap()})
    } else {
        HttpResponse::InternalServerError().json("Some error occured while evaluating")
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Graph {
    nodes: Vec<FlowNode>,
    edges: Vec<FlowEdge>
}

#[derive(Debug, Deserialize, Serialize)]
struct FlowNode {
    id: String,
    position: Position,
    data: Data,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Position {
    x: f64,
    y: f64
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Data {
    label: String,
    description: String,
    value: Option<String>,
    predicate: Predicate
}

#[derive(Debug, Deserialize, Serialize)]
struct FlowEdge {
    id: String,
    source: String,
    target: String,
    label: Direction
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
enum Direction {
   LEFT,
   RIGHT
}

#[derive(Debug, Serialize)]
struct EvaluateResponse {
    result: String
}