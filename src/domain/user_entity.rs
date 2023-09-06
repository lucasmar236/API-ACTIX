#[derive(Debug,Clone)]
pub struct UserEntity{
    pub id_user : i32,
    pub first_name : String,
    pub last_name: String,
    pub password: String,
    pub email: String,
}

impl UserEntity {
    pub fn new(
        id_user : i32,
        first_name : String,
        last_name: String,
        password: String,
        email: String,
    ) -> Self{
        UserEntity{id_user,first_name,last_name,password,email }
    }
}