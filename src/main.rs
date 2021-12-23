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
use biz::app::cruise::article::article_controller;
use biz::app::cruise::channel::channel_controller;
use biz::app::app_controller;

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
            home_controller::overview,
            home_controller::trend_overview
        ])
        .mount("/manage/app",routes![
            app_controller::page
        ])
        .mount("/manage/app/cruise/channel", routes![
            channel_controller::page
        ])
        .mount("/manage/admin/user",routes![
        ])
        .mount("/manage/app/music/fav",routes![
            fav_music_controller::page
        ])
        .mount("/manage/app/cruise/article", routes![
            article_controller::page,
            article_controller::detail
        ])
}