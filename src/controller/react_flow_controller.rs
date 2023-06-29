use actix_web::{web, Responder, HttpResponse, get, put};
use mongodb::bson::Uuid;
use crate::app_data::AppData;
use crate::domain::decision_tree::DecisionTree;
use crate::dto::react_flow_dtos::SaveDecisionTreeFromFlowRequest;
use crate::error::AppError;

#[get("/react-flow/decision_trees/{_id}")]
pub async fn get_as_flow(_id: web::Path<String>, data: web::Data<AppData>) -> impl Responder {
    let react_flow_service = &data.react_flow_service;
    let dt_service = &data.decision_tree_service;

    match Uuid::parse_str(_id.to_string()) {
        Ok(_id) => {
            match dt_service.get_decision_tree_by_id(&_id).await {
                Ok(decision_tree) => {
                    let graph = react_flow_service.get_graph(decision_tree.root).await;
                    HttpResponse::Ok().json(graph)
                },
                Err(err) => HttpResponse::InternalServerError().json(err)
            }
        },
        Err(err) => HttpResponse::BadRequest().json(AppError::GetDecisonTreeFailed { message: err.to_string() })
    }
}

#[put("/react-flow/decision_trees")]
pub async fn save_from_flow(request: web::Json<SaveDecisionTreeFromFlowRequest>, data: web::Data<AppData>) -> impl Responder {
    let _id = request._id;
    let graph = &request.graph;
    let context = &request.context;
    let dt_service = &data.decision_tree_service;
    let react_flow_service = &data.react_flow_service;

    match react_flow_service.construct_root(&graph) {
        Some(root) => {
            match dt_service.upsert_decision_tree(&DecisionTree{ _id, root, context: context.clone() }).await {
                Ok(_) => HttpResponse::Created().finish(),
                Err(err) => err.to_http_response()
            }
        },
        None => HttpResponse::BadRequest().json(AppError::SaveDecisionTreeFailed{ message: "Invalid graph provided: Could not find a root node".to_string() })
    }
}