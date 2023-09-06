use std::{env};
use std::net::TcpListener;
use actix_web::{dev::Server, middleware::Logger, web};
use actix_web::{App, HttpServer};
use crate::adapters;
use crate::adapters::{
    spi::{
      db_conn::DbConn,
      db_user_repository::UserRepository
    },
    api::shared::app_state::AppState
};


pub fn server(listener: TcpListener, db: String) -> Result<Server,std::io::Error>{
    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "actix_web=debug");
    
    env_logger::try_init().expect("Error in the env file");

    let db_conn = DbConn{ db_name: db};

    let data = web::Data::new(AppState{
        name: String::from("Actix API"),
        user_repository: UserRepository { db_conn },
    });

    let server = HttpServer::new(move ||
        App::new()
            .app_data(data.clone())
            .wrap(Logger::default())
            .configure(adapters::api::shared::routes::routes)
        )
        .listen(listener)?
        .run();

    println!("Server running on port {}", 8080);

    Ok(server)
}
