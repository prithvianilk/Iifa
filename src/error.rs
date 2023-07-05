use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum AppError {
    GetDecisonTreeFailed{message: String},
    SaveDecisionTreeFailed{message: String},
    DecisionTreeNotFound{message: String},
    EvaluateDecisionTreeFailed{message: String}
}

impl AppError {
    pub fn to_http_response(&self) -> HttpResponse {
        match self {
            AppError::GetDecisonTreeFailed { message: _ } => HttpResponse::InternalServerError().json(self),
            AppError::DecisionTreeNotFound { message: _ } => HttpResponse::NotFound().json(self),
            AppError::SaveDecisionTreeFailed { message: _ } => HttpResponse::InternalServerError().json(self),
            AppError::EvaluateDecisionTreeFailed { message: _ } => HttpResponse::InternalServerError().json(self)
        }
    }
}