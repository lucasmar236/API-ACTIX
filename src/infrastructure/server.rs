use std::{env};
use std::net::TcpListener;
use actix_web::{dev::Server, middleware::Logger};
use actix_web::{App, HttpServer};
use crate::adapters;

pub fn server(listener: TcpListener) -> Result<Server,std::io::Error>{
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "actix_web=debug");

    let server = HttpServer::new(move ||
        App::new()
            .wrap(Logger::default())
            .configure(adapters::api::shared::routes::routes)
        )
        .listen(listener)?
        .run();

    println!("Server running on port {}", 8080);

    Ok(server)
}
