use actix_web::{HttpServer, App};
use actix_cors::Cors;

mod node;
mod dt_dao;
mod customer_params;
mod predicate;
mod services;

use services::{get, save, evaluate};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cors = Cors::default().supports_credentials();
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    // Customize CORS options if needed
                    .allow_any_origin()
            )
            .service(get)
            .service(save)
            .service(evaluate)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}