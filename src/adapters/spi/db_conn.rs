use std::fmt::format;
use diesel::{pg::PgConnection, r2d2::ConnectionManager};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct DbConn{
    pub db_name: String
}

impl DbConn{
    pub fn get_pool(&self) -> DbPool{
        let url = dotenv::var("DATABASE_URL").expect("DATABASE_URL not defined");
        let manager = ConnectionManager::<PgConnection>::new(
            &format!("{}/{}",url,&self.db_name)
        );
        r2d2::Pool::new(manager).unwrap()
    }
}