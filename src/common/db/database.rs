use diesel::Connection;
use rust_wheel::config::db::config;

pub fn get_conn() -> diesel::pg::PgConnection{
    let mut connection = config::establish_connection();
    return connection;
}
