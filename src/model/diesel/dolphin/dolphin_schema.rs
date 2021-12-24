table! {
    admin_users (id) {
        id -> Int8,
        nickname -> Nullable<Varchar>,
        avatar_url -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
        updated_time -> Nullable<Int8>,
        created_time -> Nullable<Int8>,
        salt -> Nullable<Varchar>,
        pwd -> Nullable<Varchar>,
        sex -> Nullable<Int4>,
        level_type -> Nullable<Varchar>,
        phone_region -> Nullable<Varchar>,
        country_code -> Nullable<Int4>,
        user_status -> Nullable<Varchar>,
    }
}

table! {
    apps (id) {
        id -> Int4,
        app_name -> Varchar,
        remark -> Nullable<Varchar>,
        created_time -> Int8,
        updated_time -> Nullable<Int8>,
        user_count -> Nullable<Int4>,
        online_status -> Nullable<Int4>,
        online_time -> Nullable<Int8>,
        app_tag -> Nullable<Varchar>,
        app_id -> Int4,
    }
}

table! {
    article (id) {
        id -> Int8,
        user_id -> Int8,
        title -> Varchar,
        author -> Varchar,
        guid -> Varchar,
        created_time -> Int8,
        updated_time -> Int8,
        link -> Nullable<Varchar>,
        pub_time -> Nullable<Timestamptz>,
        sub_source_id -> Int8,
        cover_image -> Nullable<Varchar>,
        channel_reputation -> Int4,
        editor_pick -> Nullable<Int4>,
    }
}

table! {
    article_content (id) {
        id -> Int8,
        article_id -> Int8,
        #[sql_name = "article_content"]
        articleContent -> Varchar,
    }
}

table! {
    dashboard (id) {
        id -> Int4,
        app_count -> Int4,
        user_count -> Int4,
    }
}

table! {
    rss_sub_source (id) {
        id -> Int8,
        sub_url -> Varchar,
        created_time -> Int8,
        updated_time -> Int8,
        sub_status -> Int2,
        rss_type -> Varchar,
        standard_type -> Varchar,
        standard_version -> Varchar,
        cron -> Varchar,
        trigger_count -> Int4,
        next_trigger_time -> Nullable<Timestamp>,
        sub_name -> Varchar,
        last_trigger_time -> Nullable<Timestamptz>,
        tags -> Nullable<Array<Int4>>,
        source_url -> Nullable<Varchar>,
        sub_type -> Nullable<Varchar>,
        intro -> Nullable<Varchar>,
        remark -> Nullable<Varchar>,
        title_hash -> Nullable<Varchar>,
        failed_count -> Int4,
        lang -> Nullable<Varchar>,
        frequency_month -> Nullable<Int4>,
        reputation -> Nullable<Int4>,
        rep_lastest_refresh_time -> Nullable<Int8>,
        scrapy_take_time -> Nullable<Int4>,
        follower -> Nullable<Int8>,
        censor_status -> Nullable<Int4>,
        etag -> Nullable<Varchar>,
        last_modified -> Nullable<Varchar>,
        editor_pick -> Nullable<Int4>,
        fav_icon_url -> Nullable<Varchar>,
        dynamic_interval -> Int4,
        local_icon_url -> Nullable<Varchar>,
        creator -> Int8,
    }
}

table! {
    trend (id) {
        id -> Int8,
        trend_item -> Int4,
        app_id -> Int4,
        created_time -> Int8,
        updated_time -> Int8,
        statistic_time -> Int8,
        incre_num -> Int4,
    }
}

table! {
    users (id) {
        id -> Int8,
        nickname -> Varchar,
        avatar_url -> Nullable<Varchar>,
        phone -> Varchar,
        updated_time -> Int8,
        created_time -> Int8,
        salt -> Varchar,
        pwd -> Varchar,
        sex -> Nullable<Int4>,
        level_type -> Nullable<Varchar>,
        phone_region -> Nullable<Varchar>,
        country_code -> Nullable<Int4>,
        user_status -> Int4,
        last_login_time -> Nullable<Int8>,
        first_login_time -> Nullable<Int8>,
        app_id -> Int4,
        register_time -> Int8,
        apple_iap_product_id -> Nullable<Varchar>,
        auto_renew_product_expire_time_ms -> Nullable<Int8>,
    }
}

allow_tables_to_appear_in_same_query!(
    admin_users,
    apps,
    article,
    article_content,
    dashboard,
    rss_sub_source,
    trend,
    users,
);
