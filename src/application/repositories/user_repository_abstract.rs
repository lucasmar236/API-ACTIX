use async_trait:: async_trait;

use crate::domain::user_entity::UserEntity;

#[cfg(test)]
use mockall::{predicate::*,*};
use std::error::Error;

#[cfg_attr(test,automock)]
#[async_trait(?Send)]
pub trait UserRepositoryAbstract{
    async fn get_all_users(&self) -> Result<Vec<UserEntity>, Box<dyn Error>>;
}