use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;

use crate::model::diesel::dolphin::dolphin_models::RssSubSource;
use crate::model::request::app::cruise::channel::channel_request::ChannelRequest;
use crate::model::request::app::cruise::channel::pick_channel_request::PickChannelRequest;
use crate::service::app::cruise::channel::channel_service::{channel_query, editor_pick_channel};

#[post("/v1/page", data = "<request>")]
pub fn page(request: Json<ChannelRequest>) -> content::Json<String> {
    let channels = channel_query::<Vec<RssSubSource>>(&request);
    return box_rest_response(channels);
}

///
/// why using put?
/// https://coolshell.cn/articles/22173.html
///
#[PUT("/v1/pick", data = "<request>")]
pub fn editor_pick(request: Json<PickChannelRequest>) -> content::Json<String> {
    editor_pick_channel(request.channelId);
    return box_rest_response("ok");
}