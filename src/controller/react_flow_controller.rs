use actix_web::{web, Responder, HttpResponse, get, put};
use crate::app_data::AppData;
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
    HttpResponse::Created()
}