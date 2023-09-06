use std::net::TcpListener;
use api_actix::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let listener = TcpListener::bind("0.0.0.0:8080").expect("Failed to bind port");
    run(listener)?.await
}


