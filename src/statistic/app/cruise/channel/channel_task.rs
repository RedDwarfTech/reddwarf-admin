use std::time::Duration;

use rust_wheel::common::util::time_util::{end_of_today, get_current_millisecond, start_of_today};
use tokio::time;

use crate::model::diesel::dolphin::custom_dolphin_models::TrendAdd;
use crate::model::diesel::dolphin::dolphin_models::RssSubSource;
use crate::service::app::cruise::article::article_fav_service::channel_fav_count;
use crate::service::app::cruise::article::article_service::get_article_count_by_channel_id;
use crate::service::app::cruise::article::static_article_service::get_article_count_by_time;
use crate::service::app::cruise::channel::statistic_channel_service::{get_low_quality_channels, get_refresh_channels, get_refresh_channels_for_article, update_channel_article_count, update_channel_reputation};
use crate::service::app::cruise::overview::cruise_overview_service::{delete_low_quality_channel, update_days_article_count};

pub fn refresh_channel_reputation() -> Result<(), AnalysisError> {
    let channels: Vec<RssSubSource> = get_refresh_channels();
    if channels.is_empty() {
        return Ok(());
    }
    for channel in channels {
        let result = channel_fav_count(&channel.id);
        update_channel_reputation(result, channel.id)
    }
    Ok(())
}

pub async fn refresh_channel_rep() {
    let mut interval = time::interval(Duration::from_millis(250000));
    loop {
        interval.tick().await;
        match refresh_channel_reputation(){
            Ok(_) => (),
            _ => println!("refresh channel article failed")
        };
    }
}

pub async fn refresh_channel_article_count() {
    let mut interval = time::interval(Duration::from_millis(25000));
    loop {
        interval.tick().await;
        match refresh_channel_article() {
            Ok(_) => (),
            _ => println!("refresh channel article failed")
        }
    }
}

pub async fn remove_low_quality_articles() {
    let mut interval = time::interval(Duration::from_millis(25000));
    loop {
        interval.tick().await;
        remove_articles();
    }
}

pub async fn calculate_article_trend() {
    let mut interval = time::interval(Duration::from_secs(7200));
    loop {
        interval.tick().await;
        // today
        calculate_trend_impl(start_of_today(),end_of_today());
        // yesterday
        calculate_trend_impl(start_of_today() - 24*60*60*1000,end_of_today() - 24*60*60*1000);
    }
}

#[derive(Debug)]
pub enum AnalysisError {
    Failed,
}

pub fn refresh_channel_article() -> Result<(), AnalysisError> {
    let channels: Vec<RssSubSource> = get_refresh_channels_for_article();
    if channels.is_empty() {
        return Ok(());
    }
    for channel in channels {
        let result = get_article_count_by_channel_id(&channel.id);
        update_channel_article_count(result, channel.id)
    }
    Ok(())
}

pub fn remove_articles() {
    let channels: Vec<RssSubSource> = get_low_quality_channels();
    for channel in channels {
        delete_low_quality_channel(channel.id);
    }
}

pub fn calculate_trend_impl(start: i64, end: i64) {
    let article_count = get_article_count_by_time(start,end);
    let new_trend = TrendAdd{
        trend_item: 1,
        app_id: 1,
        created_time: get_current_millisecond(),
        updated_time: get_current_millisecond(),
        statistic_time: end_of_today(),
        incre_num: article_count as i32
    };
    update_days_article_count(&new_trend);
}