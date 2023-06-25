use std::fs::File;
use std::io::Read;

use actix_web::{web, Responder, HttpResponse, get, put, post};
use serde::Serialize;
use serde_json::Value;
use crate::app_data::AppData;
use crate::domain::node::Node;

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
    let context = read();
    let result = dt_service.get_dt().traverse(&input_params, &context);
    result.map(|result| HttpResponse::Ok().json(EvaluateResponse{result}))
          .unwrap_or(HttpResponse::InternalServerError().json("Some error occured while evaluating"))
}

fn read() -> Value {
    let mut contents = String::new();
    let mut file = File::open("context.json").expect("Failed to open file");
    file.read_to_string(&mut contents).expect("Failed to read file");
    let value: Value = serde_json::from_str(&contents).expect("Failed to deserialize JSON");
    drop(file);
    value
}

#[derive(Debug, Serialize)]
struct EvaluateResponse {
    result: String
}