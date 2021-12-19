#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

mod biz;
mod model;
mod service;
mod models;
mod test;

use rocket::{Build, Rocket};
use biz::common::health_controller;
use biz::home::home_controller;
use biz::app::music::fav::fav_music_controller;

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
        .mount("/manage/home",routes![
            home_controller::overview
        ])
        .mount("/manage/admin/user",routes![
        ])
        .mount("/manage/app/music/fav",routes![
            fav_music_controller::page
        ])
}