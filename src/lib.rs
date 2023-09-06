use std::net::TcpListener;
use actix_web::dev::Server;

mod infrastructure;
mod adapters;
mod domain;


pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    infrastructure::server::server(listener)
}