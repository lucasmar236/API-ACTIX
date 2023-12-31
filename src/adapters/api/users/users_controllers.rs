use actix_web::{web,get,HttpResponse};
use crate::{
    adapters::api::{
        users::
    }
}
pub fn routes(cfg: &mut web::ServiceConfig){
    cfg
        .service(hello);
}

#[get("")]
async fn hello() -> Result<HttpResponse,std::io::Error> {
    Ok(HttpResponse::Ok().body("Hello world!"))
}