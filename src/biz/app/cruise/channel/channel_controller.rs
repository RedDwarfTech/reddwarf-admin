use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;
use rust_wheel::model::user::login_user_info::LoginUserInfo;

use crate::model::diesel::dolphin::dolphin_models::RssSubSource;
use crate::model::request::app::cruise::channel::channel_request::ChannelRequest;
use crate::model::request::app::cruise::channel::pick_channel_request::PickChannelRequest;
use crate::model::request::app::cruise::channel::tag_channel_request::TagChannelRequest;
use crate::model::request::app::cruise::channel::update_channel_request::UpdateChannelRequest;
use crate::service::app::cruise::channel::channel_service::{channel_query, editor_pick_channel, update_channel};
use crate::service::app::cruise::channel::channel_service::update_channel_tags;

#[post("/v1/page", data = "<request>")]
pub fn page(request: Json<ChannelRequest>, login_user_info: LoginUserInfo) -> content::Json<String> {
    let channels = channel_query::<Vec<RssSubSource>>(&request, login_user_info);
    return box_rest_response(channels);
}

#[put("/v1/update", data = "<request>")]
pub fn update(request: Json<UpdateChannelRequest>) -> content::Json<String> {
    update_channel(request);
    return box_rest_response("ok");
}

///
/// why using put?
/// https://coolshell.cn/articles/22173.html
///
#[put("/v1/pick", data = "<request>")]
pub fn editor_pick(request: Json<PickChannelRequest>) -> content::Json<String> {
    editor_pick_channel(request.channelId, 1);
    return box_rest_response("ok");
}

#[put("/v1/unpick", data = "<request>")]
pub fn editor_unpick(request: Json<PickChannelRequest>) -> content::Json<String> {
    editor_pick_channel(request.channelId, 0);
    return box_rest_response("ok");
}

#[put("/v1/tags", data = "<request>")]
pub fn tags(request: Json<TagChannelRequest>) -> content::Json<String> {
    update_channel_tags(&request.channelId, request.tags.to_string());
    return box_rest_response("ok");
}
