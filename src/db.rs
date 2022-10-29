use std::env;

use diesel::pg::PgConnection;
use diesel::Connection as DieselConnection;

pub type Connection = PgConnection;

pub fn conn() -> Connection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    Connection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
