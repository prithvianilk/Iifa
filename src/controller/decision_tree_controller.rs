use actix_web::{web, Responder, HttpResponse, get, put, post};
use mongodb::bson::Uuid;
use crate::app_data::AppData;
use crate::domain::decision_tree::DecisionTree;
use crate::dto::decision_tree_dtos::{UpdateDecisionTreeRequest, EvaluateRequest, EvaluateResponse, CreateDecisionTreeRequest};
use crate::error::AppError;

// Improvement: Fetch only id and desc
#[get("/decision_trees")]
pub async fn get_all_decision_trees(data: web::Data<AppData>) -> impl Responder {
    let decision_tree_service = &data.decision_tree_service;
    
    match decision_tree_service.get_all_decision_trees().await {
        Ok(decision_trees) => HttpResponse::Ok().json(decision_trees),
        Err(err) => err.to_http_response()
    }
}

#[get("/decision_trees/{_id}")]
pub async fn get_by_id(_id: web::Path<String>, data: web::Data<AppData>) -> impl Responder {
    let decision_tree_service = &data.decision_tree_service;

    match Uuid::parse_str(_id.to_string()) {
        Ok(_id) => {
            decision_tree_service.get_decision_tree_by_id(&_id).await
                       .map(|decision_tree| HttpResponse::Ok().json(decision_tree))
                       .unwrap_or(HttpResponse::NotFound().json(AppError::GetDecisonTreeFailed{ message: "No decision tree with this id".to_string() }))
        },
        Err(err) => HttpResponse::BadRequest().json(AppError::GetDecisonTreeFailed{ message:  err.to_string()})
    }
}

#[post("/decision_trees")]
pub async fn create(request: web::Json<CreateDecisionTreeRequest>, data: web::Data<AppData>) -> impl Responder {
    let decision_tree_service = &data.decision_tree_service;
    
    let decision_tree = &DecisionTree { 
        _id: Uuid::new(), 
        root: request.root.clone(), 
        description: request.description.clone(), 
        context: request.context.clone() 
    };
    match decision_tree_service.upsert_decision_tree(&decision_tree).await {
        Ok(_) => HttpResponse::Created().json(decision_tree),
        Err(err) => err.to_http_response()
    }
}

#[put("/decision_trees")]
pub async fn update(request: web::Json<UpdateDecisionTreeRequest>, data: web::Data<AppData>) -> impl Responder {
    let decision_tree_service = &data.decision_tree_service;

    match Uuid::parse_str(request._id.to_string()) {
        Ok(_id) => {
            let decision_tree = &DecisionTree { 
                _id, 
                root: request.root.clone(), 
                description: request.description.clone(), 
                context: request.context.clone() 
            };
            match decision_tree_service.upsert_decision_tree(decision_tree).await {
                Ok(_) => HttpResponse::Ok().json(decision_tree),
                Err(err) => HttpResponse::InternalServerError().json(err)
            }
        },
        Err(err) => HttpResponse::BadRequest().json(AppError::SaveDecisionTreeFailed{ message:  err.to_string() })
    }
}

#[post("/decision_trees/{_id}/evaluate")]
pub async fn evaluate(request: web::Json<EvaluateRequest>, _id: web::Path<String>, data: web::Data<AppData>) -> impl Responder {
    let decision_tree_service = &data.decision_tree_service;
    let input_params = &request.input_params;

    match Uuid::parse_str(_id.to_string()) {
        Ok(_id) => {
            match decision_tree_service.get_decision_tree_by_id(&_id).await {
                Ok(mut decision_tree) => {
                    decision_tree.root.traverse(input_params, &decision_tree.context)
                                    .map(|result| HttpResponse::Ok().json(EvaluateResponse{result}))
                                    .unwrap_or(HttpResponse::InternalServerError().json(AppError::EvaluateDecisionTreeFailed { message: "No result found".to_string() }))
                },
                Err(err) => err.to_http_response()
            }
        },
        Err(err) => HttpResponse::BadRequest().json(AppError::EvaluateDecisionTreeFailed{ message:  err.to_string() })
    }
}
