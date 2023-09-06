use crate::adapters::spi::db_user_repository::UserRepository;

pub struct AppState{
    pub name : String,
    pub user_repository: UserRepository
}