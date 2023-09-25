use rust_wheel::config::db::config;

pub fn get_conn() -> diesel::pg::PgConnection{
    let connection = config::establish_connection();
    return connection;
}