use async_trait::async_trait;
use diesel::prelude::*;
use std::error::Error;

use crate::adapters::spi::{
    db_conn::DbConn,db_mappers::UserMapper,models::Users,schema::users::dsl::*
};
use crate::application::{
    mappers::db_mapper::DbMapper,
    repositories::user_repository_abstract::UserRepositoryAbstract};
use crate::domain::user_entity::UserEntity;


pub struct UserRepository{
    pub db_conn: DbConn
}

#[async_trait(?Send)]
impl UserRepositoryAbstract for UserRepository{
    async fn get_all_users(&self) -> Result<Vec<UserEntity>,Box<dyn Error>>{
        let conn = self
            .db_conn.get_pool().get().expect("Couldn't get connection");

        let results = users.load::<Users>(&conn);

        match results {
            Ok(models) => Ok(models
                .into_iter()
                .map(UserMapper::entity).
                collect::<Vec<UserEntity>>()
            ),
            Err(e) => Err(Box::new(e))
        }
    }
}