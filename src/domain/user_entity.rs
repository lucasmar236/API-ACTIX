#[derive(Debug,Clone)]
pub struct UserEntity{
    pub first_name : String,
    pub last_name: String,
    pub password: String,
    pub email: String,
}

impl UserEntity {
    pub fn new(
        first_name : String,
        last_name: String,
        password: String,
        email: String,
    ) -> Self{
        UserEntity{ first_name,last_name,password,email }
    }
}