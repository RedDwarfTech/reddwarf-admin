#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

mod biz;
mod model;
mod service;
mod models;
mod test;

use futures::stream::{self, StreamExt};
use std::time::Duration;
use rocket::{Build, Rocket};
use rocket::tokio::time::Instant;
use tokio::time;
use biz::common::health_controller;
use biz::home::home_controller;
use biz::app::music::fav::fav_music_controller;
use biz::app::cruise::article::article_controller;
use biz::app::cruise::channel::channel_controller;
use biz::user::user_controller;
use biz::app::app_controller;
use biz::app::cernitor::domain::domain_controller;

#[launch]
#[tokio::main]
async fn rocket() -> _ {
    period_exec();
    build_rocket()
}

async fn period_exec(){
    let interval = time::interval(Duration::from_millis(10));

    let forever = stream::unfold(interval, |mut interval| async {
        interval.tick().await;
        do_something().await;
        Some(((), interval))
    });

    let now = Instant::now();
    forever.for_each(|_| async {}).await;
}

async fn do_something() {
    eprintln!("do_something");
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
        .mount("/manage/app/cernitor/domain",routes![
            domain_controller::page
        ])
        .mount("/manage/app/cruise/channel", routes![
            channel_controller::page
        ])
        .mount("/manage/user",routes![
            user_controller::page,
            user_controller::edit_pwd
        ])
        .mount("/manage/app/music/fav",routes![
            fav_music_controller::page
        ])
        .mount("/manage/app/cruise/article", routes![
            article_controller::page,
            article_controller::detail
        ])
}