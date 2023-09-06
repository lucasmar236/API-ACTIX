use std::net::TcpListener;
use actix_web::dev::Server;

mod infrastructure;
mod adapters;
mod domain;
mod application;

extern crate dotenv;
extern crate log;

#[macro_use]
extern crate diesel;
extern crate r2d2;

pub fn run(listener: TcpListener, db: String) -> Result<Server, std::io::Error> {
    infrastructure::server::server(listener,db)
}