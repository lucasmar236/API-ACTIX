use crate::adapters::spi::schema::users;
#[derive(Queryable,QueryableByName)]
#[table_name="users"]
pub struct Users{
    pub id: i32,
    pub first_name : String,
    pub last_name: String,
    pub password: String,
    pub email: String,
}

