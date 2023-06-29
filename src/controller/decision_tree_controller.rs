use actix_web::{web, Responder, HttpResponse, get, put, post};
use mongodb::bson::Uuid;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::app_data::AppData;
use crate::domain::decision_tree::DecisionTree;
use crate::domain::node::Node;
use crate::util::file_util::read_file;
use crate::error::AppError;

#[get("/decision_trees/{_id}")]
pub async fn get(_id: web::Path<String>, data: web::Data<AppData>) -> impl Responder {
    let dt_service = &data.decision_tree_service;

    match Uuid::parse_str(_id.to_string()) {
        Ok(_id) => {
            dt_service.get_decision_tree_by_id(&_id).await
                       .map(|decision_tree| HttpResponse::Ok().json(decision_tree))
                       .unwrap_or(HttpResponse::NotFound().json(AppError::GetDecisonTreeFailed{ message: "No decision tree with this id".to_string() }))
        },
        Err(err) => HttpResponse::BadRequest().json(AppError::GetDecisonTreeFailed{ message:  err.to_string()})
    }
}

#[put("/decision_trees")]
pub async fn upsert(request: web::Json<UpdateDecisionTree>, data: web::Data<AppData>) -> impl Responder {
    let dt_service = &data.decision_tree_service;
    
    match Uuid::parse_str(request._id.to_string()) {
        Ok(_id) => {
            let decision_tree = &DecisionTree { _id, root: request.root.clone() };
            match dt_service.upsert_decision_tree(decision_tree).await {
                Ok(_) => HttpResponse::Ok().json(decision_tree),
                Err(err) => HttpResponse::InternalServerError().json(err)
            }
        },
        Err(err) => HttpResponse::BadRequest().json(AppError::SaveDecisionTreeFailed{ message:  err.to_string() })
    }
}

#[post("/decision_trees/evaluate")]
pub async fn evaluate(request: web::Json<EvaluateRequest>, data: web::Data<AppData>) -> impl Responder {
    let dt_service = &data.decision_tree_service;
    let input_params = &request.input_params;
    let context = read_file(&"context.json".to_string());
    let result = dt_service.get_decision_tree_by_id(&request._id).await.unwrap().root
                           .traverse(input_params, &context);
    result.map(|result| HttpResponse::Ok().json(EvaluateResponse{result}))
          .unwrap_or(HttpResponse::InternalServerError().json(AppError::EvaluateDecisionTreeFailed { message: "No result found".to_string() }))
}

#[derive(Debug, Deserialize)]
pub struct UpdateDecisionTree {
    pub _id: String,
    pub root: Node
}

#[derive(Debug, Deserialize)]
pub struct EvaluateRequest {
    pub _id: Uuid,
    pub input_params: Value
}

#[derive(Debug, Serialize)]
struct EvaluateResponse {
    pub result: String
}