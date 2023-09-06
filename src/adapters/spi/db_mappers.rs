use crate::adapters::spi::models::Users;
use crate::application::mappers::db_mapper::DbMapper;
use crate::domain::user_entity::UserEntity;

pub struct UserMapper {}

impl DbMapper<UserEntity,Users> for UserMapper{
    fn db(entity: UserEntity) -> Users {
        Users{
             id: entity.id_user,
             first_name : entity.first_name,
             last_name: entity.last_name,
             password: entity.password,
             email: entity.email,
        }
    }

    fn entity(model: Users) -> UserEntity {
        UserEntity{
            id_user: model.id,
            first_name: model.first_name,
            last_name: model.last_name,
            password: model.password,
            email: model.email
        }
    }
}