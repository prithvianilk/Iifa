use actix_web::{HttpServer, App};

mod node;
mod dt_dao;
mod customer_params;
mod predicate;
mod services;

use services::{get, save, evaluate};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(get)
            .service(save)
            .service(evaluate)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}