use crate::handlers::{handle_graphiql, handle_graphql};
use crate::schema::create_schema;
use actix_web::{middleware, web, App, HttpServer};
use std::io;
use std::sync::Arc;

mod handlers;
mod schema;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let schema = Arc::new(create_schema());

    let host = "0.0.0.0";
    let port = 8000;

    let server = HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/graphql").route(web::post().to(handle_graphql)))
            .service(web::resource("/graphiql").route(web::get().to(handle_graphiql)))
    })
    .bind((host, port))?
    .run();

    eprintln!("{}", format!("Listening on {}:{}...", host, port));

    server.await
}
