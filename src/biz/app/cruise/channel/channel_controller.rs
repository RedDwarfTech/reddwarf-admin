use okapi::openapi3::OpenApi;
use rocket::response::content;
use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes_spec};
use rocket_okapi::settings::OpenApiSettings;
use rust_wheel::common::util::model_convert::box_rest_response;
use rust_wheel::model::user::login_user_info::LoginUserInfo;

use crate::model::diesel::dolphin::dolphin_models::RssSubSource;
use crate::model::request::app::cruise::channel::channel_request::ChannelRequest;
use crate::model::request::app::cruise::channel::pick_channel_request::PickChannelRequest;
use crate::model::request::app::cruise::channel::tag_channel_request::TagChannelRequest;
use crate::model::request::app::cruise::channel::update_channel_request::UpdateChannelRequest;
use crate::service::app::cruise::channel::channel_service::{channel_query, editor_pick_channel, update_channel};
use crate::service::app::cruise::channel::channel_service::update_channel_tags;

pub fn get_routes_and_docs(settings: &OpenApiSettings) -> (Vec<rocket::Route>, OpenApi) {
    openapi_get_routes_spec![settings: page, update,editor_pick,editor_unpick, tags]
}

#[openapi(tag = "频道")]
#[get("/v1/page?<query..>")]
pub fn page(query: ChannelRequest, login_user_info: LoginUserInfo) -> content::RawJson<String> {
    let channels = channel_query::<Vec<RssSubSource>>(&query, login_user_info);
    return box_rest_response(channels);
}

#[openapi(tag = "频道")]
#[put("/v1/update", data = "<request>")]
pub fn update(request: Json<UpdateChannelRequest>) -> content::RawJson<String> {
    update_channel(request);
    return box_rest_response("ok");
}

///
/// why using put?
/// https://coolshell.cn/articles/22173.html
///
#[openapi(tag = "频道")]
#[put("/v1/pick", data = "<request>")]
pub fn editor_pick(request: Json<PickChannelRequest>) -> content::RawJson<String> {
    editor_pick_channel(request.channelId, request.editor_pick);
    return box_rest_response("ok");
}

#[openapi(tag = "频道")]
#[put("/v1/unpick", data = "<request>")]
pub fn editor_unpick(request: Json<PickChannelRequest>) -> content::RawJson<String> {
    editor_pick_channel(request.channelId, 0);
    return box_rest_response("ok");
}

#[openapi(tag = "频道")]
#[put("/v1/tags", data = "<request>")]
pub fn tags(request: Json<TagChannelRequest>) -> content::RawJson<String> {
    update_channel_tags(&request.channelId, request.tags.to_string());
    return box_rest_response("ok");
}
