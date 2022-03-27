use crate::model::diesel::dolphin::dolphin_models::RssSubSource;
use crate::service::app::cruise::article::article_fav_service::channel_fav_count;
use crate::service::app::cruise::channel::statistic_channel_service::{get_refresh_channels, update_channel_reputation};

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


