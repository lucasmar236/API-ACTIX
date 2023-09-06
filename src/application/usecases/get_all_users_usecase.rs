use std::error::Error;
use async_trait::async_trait;

use crate::{
    application::{
        repositories::user_repository_abstract::UserRepositoryAbstract,
        usecases::get_all_users_usecase
    },
    domain::{user_entity::UserEntity,error::RespError}
};
use crate::application::usecases::interfaces::AbstractUseCase;
use crate::application::utils::error_utils::ErrorUtils;


pub struct GetAllUserUseCase <'a>{
    repository: &'a dyn UserRepositoryAbstract
}

impl <'a> GetAllUserUseCase<'a>{
    pub fn new(repository: &'a dyn UserRepositoryAbstract) -> Self{
        GetAllUserUseCase{repository}
    }
}

#[async_trait(?Send)]
impl <'a> AbstractUseCase<Vec<UserEntity>> for GetAllUserUseCase<'a>{
    async fn execute(&self) -> Result<Vec<UserEntity>, RespError> {
        let users = self.repository.get_all_users().await;
        
        match users {
            Ok(all_user) => Ok(all_user),
            Err(e) =>
                Err(ErrorUtils::bad_request_error("Cannot get all users",Some(e)))
        }
    }
}