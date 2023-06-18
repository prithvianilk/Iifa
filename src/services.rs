use actix_web::{web, Responder, HttpResponse, get, put, post};
use serde::Serialize;

use crate::{dt_dao::DtDao, node::{Node}, customer_params::CustomerParams};

const STEP: i32 = 150;

#[get("/dt")]
pub async fn get() -> impl Responder {
    let dao = DtDao::new("dt.json");
    let root = dao.get_root();
    let mut nodes: Vec<FlowNode> = Vec::new();
    let mut edges: Vec<FlowEdge>= Vec::new();
    populate_nodes_and_edges(&root, 1, &Position { x: 0, y: 0 }, &mut nodes, &mut edges);
    HttpResponse::Ok().json(GetResponse{nodes, edges})
}

fn populate_nodes_and_edges(root: &Node, id: u32, position: &Position, nodes: &mut Vec<FlowNode>, edges: &mut Vec<FlowEdge>) {
    let is_termial = root.value.is_some();
    if is_termial {
        nodes.push(FlowNode{id: id.to_string(), position: position.clone(), data: Data { label: root.value.clone().unwrap() }});
        return
    }
    nodes.push(FlowNode{id: id.to_string(), position: position.clone(), data: Data { label: root.description.clone() }});
    if let Some(left) = &root.left {
        let edge_id = format!("{}-{}", id, id * 2);
        edges.push(FlowEdge { id: edge_id, source: id.to_string(), target: (id * 2).to_string() });
        populate_nodes_and_edges(left, id * 2, &Position { x: position.x + STEP, y: position.y + STEP }, nodes, edges)
    }
    if let Some(right) = &root.right {
        let edge_id = format!("{}-{}", id, id * 2 + 1);
        edges.push(FlowEdge { id: edge_id, source: id.to_string(), target: (id * 2 + 1).to_string() });
        populate_nodes_and_edges(right, id * 2 + 1, &Position { x: position.x - STEP, y: position.y + STEP }, nodes, edges)
    }
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

#[derive(Debug, Serialize)]
struct GetResponse {
    nodes: Vec<FlowNode>,
    edges: Vec<FlowEdge>
}

#[derive(Debug, Serialize)]
struct FlowNode {
    id: String,
    position: Position,
    data: Data,
}

#[derive(Debug, Serialize, Clone)]
struct Position {
    x: i32,
    y: i32
}

#[derive(Debug, Serialize)]
struct Data {
    label: String
}

#[derive(Debug, Serialize)]
struct FlowEdge {
    id: String,
    source: String,
    target: String
}

#[derive(Debug, Serialize)]
struct EvaluateResponse {
    result: String
}