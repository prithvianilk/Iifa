use actix_web::{HttpServer, App, web};
use actix_cors::Cors;

mod controller {
    pub mod react_flow_controller;
    pub mod decision_tree_controller;
}
mod domain {
    pub mod node;
    pub mod predicate;
    pub mod decision_tree;
}
mod service {
    pub mod react_flow_service;
    pub mod decison_tree_service;
    pub mod decision_tree_dao;
}
mod dto {
    pub mod decision_tree_dtos;
    pub mod react_flow_dtos;
}
mod util {
    pub mod file_util;
}
mod error;
mod app_data;

use controller::react_flow_controller::{get_as_flow, save_from_flow};
use controller::decision_tree_controller::{get, update, evaluate, create};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = app_data::get_app_data().await;
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_data.clone()))
            .wrap(Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header() 
                    .supports_credentials())
            .service(get_as_flow)
            .service(save_from_flow)
            .service(get)
            .service(create)
            .service(update)
            .service(evaluate)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}