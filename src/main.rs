mod utils;
mod handlers;
mod responses;

use actix_web::{App, HttpServer, web};
use crate::handlers::update::update;
use crate::handlers::deploy::deploy;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting RPC server");

    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/container")
                    .route("/update", web::get().to(update))
                    .route("/{id}/deploy", web::get().to(deploy))
            )
    })
        .bind("0.0.0.0:3000")?
        .run()
        .await
}
