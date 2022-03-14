use rocket::response::content;
use rocket::serde::json::Json;
use rust_wheel::common::util::model_convert::box_rest_response;

use crate::model::diesel::dolphin::dolphin_models::RssSubSource;
use crate::model::request::app::cruise::channel::channel_request::ChannelRequest;
use crate::service::app::cruise::channel::channel_service::channel_query;

#[post("/v1/page", data = "<request>")]
pub fn page(request: Json<ChannelRequest>) -> content::Json<String> {
    let channels = channel_query::<Vec<RssSubSource>>(&request);
    return box_rest_response(channels);
}