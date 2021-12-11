#[macro_use]
extern crate rocket;

mod biz;

use rocket::{Build, Rocket};
use biz::common::health_controller;

#[launch]
fn rocket() -> _ {
    build_rocket()
}


fn build_rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/actuator", routes![
            health_controller::health,
            health_controller::liveness
        ])
}