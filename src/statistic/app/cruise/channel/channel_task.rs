use std::time::Duration;

use tokio::time;

use crate::model::diesel::dolphin::dolphin_models::RssSubSource;
use crate::service::app::cruise::article::article_fav_service::channel_fav_count;
use crate::service::app::cruise::article::article_service::get_article_count_by_channel_id;
use crate::service::app::cruise::channel::statistic_channel_service::{get_refresh_channels, get_refresh_channels_for_article, update_channel_article_count, update_channel_reputation};

pub fn refresh_channel_reputation() {
    let channels: Vec<RssSubSource> = get_refresh_channels();
    if channels.is_empty() {
        return;
    }
    for channel in channels {
        let result = channel_fav_count(&channel.id);
        update_channel_reputation(result, channel.id)
    }
}

pub async fn refresh_channel_rep(){
    let mut  interval = time::interval(Duration::from_millis(250000));
    loop {
        interval.tick().await;
        refresh_channel_reputation();
    }
}

pub async fn refresh_channel_article_count(){
    let mut  interval = time::interval(Duration::from_millis(25000));
    loop {
        interval.tick().await;
        refresh_channel_article();
    }
}

pub fn refresh_channel_article() {
    let channels: Vec<RssSubSource> = get_refresh_channels_for_article();
    if channels.is_empty() {
        return;
    }
    for channel in channels {
        let result = get_article_count_by_channel_id(&channel.id);
        update_channel_article_count(result, channel.id)
    }
}