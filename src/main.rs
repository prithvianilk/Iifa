use actix_web::{HttpServer, App};
use actix_cors::Cors;

mod node;
mod dt_dao;
mod customer_params;
mod predicate;
mod services;

use services::{get, save, evaluate, get_as_flow, save_from_flow};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
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