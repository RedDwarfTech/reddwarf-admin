use diesel::PgConnection;
use rust_wheel::config::db::config;

pub fn get_conn() -> PgConnection{
    let mut connection = config::establish_connection();
    return connection;
}
