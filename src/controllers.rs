use actix_web::{web, Responder, HttpResponse, get, put, post};
use serde::{Serialize};
use crate::app_data::AppData;
use crate::customer_params::CustomerParams;
use crate::domain::node::Node;
use crate::dto::react_flow_dtos::Graph;

#[get("/react-flow/dt")]
pub async fn get_as_flow(data: web::Data<AppData>) -> impl Responder {
    let react_flow_service = &data.react_flow_service;
    let graph = react_flow_service.get_graph();
    HttpResponse::Ok().json(graph)
}

#[put("/react-flow/dt")]
pub async fn save_from_flow(request: web::Json<Graph>, data: web::Data<AppData>) -> impl Responder {
    let graph = request.0;
    let react_flow_service = &data.react_flow_service;
    react_flow_service.save_graph_as_dt(&graph);
    HttpResponse::Ok()
}

#[get("/dt")]
pub async fn get(data: web::Data<AppData>) -> impl Responder {
    let dt_service = &data.dt_service;
    let root = dt_service.get_dt();
    HttpResponse::Ok().json(root)
}

#[put("/dt")]
pub async fn save(request: web::Json<Node>, data: web::Data<AppData>) -> impl Responder {
    let dt_service = &data.dt_service;
    dt_service.save_dt(&request.0);
    HttpResponse::Created()
}

#[post("/dt/evaluate")]
pub async fn evaluate(request: web::Json<CustomerParams>, data: web::Data<AppData>) -> impl Responder {
    let dt_service = &data.dt_service;
    let result = dt_service.get_dt().traverse(&request.0);
    result.map(|result| HttpResponse::Ok().json(EvaluateResponse{result}))
          .unwrap_or(HttpResponse::InternalServerError().json("Some error occured while evaluating"))
}

#[derive(Debug, Serialize)]
struct EvaluateResponse {
    result: String
}