use actix_web::{HttpServer, App, web};
use actix_cors::Cors;

mod controller {
    pub mod react_flow_controller;
    pub mod dt_controller;
}
mod domain {
    pub mod node;
    pub mod predicate;
    pub mod customer_params;
}
mod service {
    pub mod react_flow_service;
    pub mod dt_service;
    pub mod dt_dao;
}
mod dto {
    pub mod react_flow_dtos;
}
mod app_data;

use controller::react_flow_controller::{get_as_flow, save_from_flow};
use controller::dt_controller::{get, save, evaluate};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_data::get_app_data()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header() 
                    .supports_credentials()
            )
            .service(get_as_flow)
            .service(save_from_flow)
            .service(get)
            .service(save)
            .service(evaluate)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}