use actix_web::{web, Responder, HttpResponse, get, put, post};
use serde::Serialize;

use crate::{dt_dao::DtDao, node::Node, customer_params::CustomerParams};

#[get("/dt")]
pub async fn get() -> impl Responder {
    let dao = DtDao::new("dt.json");
    HttpResponse::Ok().json(dao.get_root())
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
struct EvaluateResponse {
    result: String
}