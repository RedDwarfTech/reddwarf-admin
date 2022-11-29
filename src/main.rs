#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};
use rocket_okapi::{mount_endpoints_and_merged_docs, OpenApiError, rapidoc::*, swagger_ui::*};
use rocket_okapi::settings::UrlObject;

use biz::app::app_controller;
use biz::app::cernitor::domain::domain_controller;
use biz::app::cruise::article::article_controller;
use biz::app::cruise::channel::channel_controller;
use biz::app::cruise::overview::cruise_trend_controller;
use biz::app::dict::translate::translate_controller;
use biz::app::dict::word::word_controller;
use biz::app::gallery::repo_app_controller;
use biz::app::iap_product_controller;
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

pub type Result<T> = std::result::Result<T, OpenApiError>;

#[rocket::main]
async fn main() {
    tokio::spawn(refresh_channel_rep());
    tokio::spawn(refresh_channel_article_count());
    tokio::spawn(remove_low_quality_articles());
    tokio::spawn(calculate_article_trend());

    let launch_result = create_server().launch().await;
    match launch_result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    };
}

pub fn create_server() -> Rocket<Build> {
    let mut building_rocket = rocket::build()
        .mount("/actuator",routes![
            health_controller::health,
            health_controller::liveness
        ])
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../fortune/openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                title: Some("My special documentation | RapiDoc".to_owned()),
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../v1/openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        );

    let openapi_settings = rocket_okapi::settings::OpenApiSettings::default();
    mount_endpoints_and_merged_docs! {
        building_rocket, "/manage".to_owned(), openapi_settings,
        "/actuator" => health_controller::get_routes_and_docs(&openapi_settings),
        "/home" => home_controller::get_routes_and_docs(&openapi_settings),
        "/sys/dict" => sys_dict_controller::get_routes_and_docs(&openapi_settings),
        "/sys/tag" => tag_controller::get_routes_and_docs(&openapi_settings),
        "/permission/menu" => menu_controller::get_routes_and_docs(&openapi_settings),
        "/permission/org" => org_controller::get_routes_and_docs(&openapi_settings),
        "/app/overview/product" => product_controller::get_routes_and_docs(&openapi_settings),
        "/app/overview/iapproduct" => iap_product_controller::get_routes_and_docs(&openapi_settings),
        "/app/overview/app" => app_controller::get_routes_and_docs(&openapi_settings),
        "/app/cruise/overview" => cruise_trend_controller::get_routes_and_docs(&openapi_settings),
        "/app/cruise/channel" => channel_controller::get_routes_and_docs(&openapi_settings),
        "/app/cruise/article" => article_controller::get_routes_and_docs(&openapi_settings),
        "/permission/user" => admin_user_controller::get_routes_and_docs(&openapi_settings),
        "/permission/role" => role_controller::get_routes_and_docs(&openapi_settings),
    };

    building_rocket
}

fn build_rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/manage/app/user",routes![
            user_controller::page,
            user_controller::edit_pwd
        ])
        .mount("/manage/app/cernitor/domain",routes![
            domain_controller::page,
            domain_controller::add,
            domain_controller::edit,
        ])
        .mount("/manage/app/music/fav",routes![
            fav_music_controller::page
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
        ])
}
