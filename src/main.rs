#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

use std::time::Duration;

use futures::stream::{self, StreamExt};
use rocket::{Build, Rocket};
use rocket::tokio::time::Instant;
use tokio::time;

use biz::app::app_controller;
use biz::app::cernitor::domain::domain_controller;
use biz::app::cruise::article::article_controller;
use biz::app::cruise::channel::channel_controller;
use biz::app::dict::translate::translate_controller;
use biz::app::dict::word::word_controller;
use biz::app::job::interview::interview_controller;
use biz::app::music::fav::fav_music_controller;
use biz::common::health_controller;
use biz::home::home_controller;
use biz::user::user_controller;
use biz::common::sys_dict_controller;

mod biz;
mod model;
mod service;
mod models;
mod test;

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
        .mount("/manage/sys",routes![
            sys_dict_controller::list
        ])
        .mount("/manage/app",routes![
            app_controller::page,
            app_controller::add,
            app_controller::edit
        ])
        .mount("/manage/app/cernitor/domain",routes![
            domain_controller::page,
            domain_controller::add
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
        .mount("/manage/app/dict/translate", routes![
            translate_controller::trans
        ])
        .mount("/manage/app/dict/word", routes![
            word_controller::glossary,
            word_controller::add_glossary
        ])
        .mount("/manage/app/job/interview",routes![
            interview_controller::page,
            interview_controller::add,
            interview_controller::update
        ])
}