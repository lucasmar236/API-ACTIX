use std::net::TcpListener;
use api_actix::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env").ok();
    let listener = TcpListener::bind("0.0.0.0:8080").expect("Failed to bind port");
    let database = dotenv::var("DATABASE_NAME").expect("Database not defined");
    run(listener,database)?.await
}


