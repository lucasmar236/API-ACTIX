use actix_web::web;

use crate::adapters::api::{users::users_controllers};


pub fn routes(config: &mut web::ServiceConfig) {
    config
        .service(web::scope("api").configure(users_controllers::routes));
}