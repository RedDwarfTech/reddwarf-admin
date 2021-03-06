#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

use biz::app::app_controller;
use biz::app::cernitor::domain::domain_controller;
use biz::app::cruise::article::article_controller;
use biz::app::cruise::channel::channel_controller;
use biz::app::cruise::overview::cruise_trend_controller;
use biz::app::dict::translate::translate_controller;
use biz::app::dict::word::word_controller;
use biz::app::gallery::repo_app_controller;
use biz::app::job::interview::interview_controller;
use biz::app::music::fav::fav_music_controller;
use biz::app::product_controller;
use biz::common::health_controller;
use biz::home::home_controller;
use biz::permission::menu::menu_controller;
use biz::permission::org::org_controller;
use biz::permission::role::role_controller;
use biz::permission::user::admin_user_controller;
use biz::system::dict::sys_dict_controller;
use biz::system::tag::tag_controller;
use biz::user::user_controller;

use crate::statistic::app::cruise::channel::channel_task::{calculate_article_trend, refresh_channel_article_count, refresh_channel_rep, remove_low_quality_articles};

mod biz;
mod statistic;
mod model;
mod service;
mod models;
mod test;
mod common;

#[launch]
#[tokio::main]
async fn rocket() -> _ {
    tokio::spawn(refresh_channel_rep());
    tokio::spawn(refresh_channel_article_count());
    tokio::spawn(remove_low_quality_articles());
    tokio::spawn(calculate_article_trend());
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
        .mount("/manage/sys/dict",routes![
            sys_dict_controller::list,
            sys_dict_controller::page,
        ])
        .mount("/manage/app/overview/product",routes![
            product_controller::page,
            product_controller::add,
            product_controller::edit,
            product_controller::get,
            product_controller::list,
        ])
        .mount("/manage/app/overview/app",routes![
            app_controller::page,
            app_controller::add,
            app_controller::edit,
            app_controller::detail,
        ])
        .mount("/manage/sys/tag",routes![
            tag_controller::list,
            tag_controller::page,
            tag_controller::add,
            tag_controller::edit,
            tag_controller::detail,
        ])
        .mount("/manage/app/user",routes![
            user_controller::page,
            user_controller::edit_pwd
        ])
        .mount("/manage/app/cernitor/domain",routes![
            domain_controller::page,
            domain_controller::add,
            domain_controller::edit,
        ])
        .mount("/manage/app/cruise/overview", routes![
            cruise_trend_controller::list
        ])
        .mount("/manage/app/cruise/channel", routes![
            channel_controller::page,
            channel_controller::editor_pick,
            channel_controller::editor_unpick,
            channel_controller::tags,
            channel_controller::update
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
        ]).mount("/manage/app/gallary/repoapp", routes![
            repo_app_controller::page,
            repo_app_controller::add,
            repo_app_controller::edit,
            repo_app_controller::detail,
        ])
        .mount("/manage/permission/role", routes![
            role_controller::page,
            role_controller::list,
            role_controller::edit_role,
            role_controller::edit_role_menu_bind,
            role_controller::get_role_menu_tree,
            role_controller::add_role
        ]).mount("/manage/permission/user", routes![
            admin_user_controller::page,
            admin_user_controller::edit_pwd,
            admin_user_controller::get_user_menu,
            admin_user_controller::get_user_roles,
            admin_user_controller::save_user_roles,
            admin_user_controller::add
        ]).mount("/manage/permission/menu", routes![
            menu_controller::page_tree,
            menu_controller::menu_tree,
            menu_controller::edit_menu,
            menu_controller::add_menu
        ]).mount("/manage/permission/org", routes![
            org_controller::page_tree,
            org_controller::org_tree,
            org_controller::edit_org,
            org_controller::org_list_query
        ])
}
