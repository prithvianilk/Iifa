use actix_web::{web, Responder, HttpResponse, get, put, post};
use serde::Serialize;
use serde_json::Value;
use crate::app_data::AppData;
use crate::domain::node::Node;
use crate::util::file_util::read_file;

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
pub async fn evaluate(request: web::Json<Value>, data: web::Data<AppData>) -> impl Responder {
    let dt_service = &data.dt_service;
    let input_params = request.0;
    let context = read_file(&"context.json".to_string());
    let result = dt_service.get_dt().traverse(&input_params, &context);
    result.map(|result| HttpResponse::Ok().json(EvaluateResponse{result}))
          .unwrap_or(HttpResponse::InternalServerError().json("Some error occured while evaluating"))
}

#[derive(Debug, Serialize)]
struct EvaluateResponse {
    result: String
}