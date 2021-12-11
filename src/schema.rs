table! {
    actor (id) {
        id -> Int8,
        actor_name -> Nullable<Varchar>,
        actor_name_intl -> Nullable<Json>,
    }
}

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
    app_log (id) {
        app_id -> Nullable<Int4>,
        created_time -> Int8,
        updated_time -> Int8,
        message -> Varchar,
        user_id -> Nullable<Int8>,
        id -> Int8,
    }
}

table! {
    apple_server_notification_record (id) {
        id -> Int8,
        created_time -> Int8,
        updated_time -> Int8,
        notification_type -> Varchar,
        auto_renew_adam_id -> Nullable<Varchar>,
        auto_renew_product_id -> Varchar,
        auto_renew_status -> Varchar,
        auto_renew_status_change_date -> Nullable<Varchar>,
        auto_renew_status_change_date_ms -> Nullable<Varchar>,
        auto_renew_status_change_date_pst -> Nullable<Varchar>,
        environment -> Varchar,
        expiration_intent -> Nullable<Int4>,
        shared_secret -> Varchar,
        bid -> Varchar,
        bvrs -> Varchar,
        unified_receipt -> Nullable<Varchar>,
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
        article_content -> Varchar,
    }
}

table! {
    article_favorites (id) {
        user_id -> Int8,
        article_id -> Int8,
        created_time -> Int8,
        updated_time -> Int8,
        fav_status -> Int4,
        upvote_status -> Int4,
        channel_id -> Int8,
        read_status -> Int4,
        read_time -> Nullable<Int8>,
        id -> Int8,
    }
}

table! {
    dashboard (id) {
        id -> Int4,
        app_count -> Int4,
        user_count -> Nullable<Int4>,
    }
}

table! {
    domain (id) {
        id -> Int8,
        domain_name -> Varchar,
        domain_url -> Varchar,
        created_time -> Int8,
        updated_time -> Int8,
        cron -> Nullable<Varchar>,
        next_trigger_time -> Nullable<Timestamp>,
        monitor_status -> Nullable<Varchar>,
        user_id -> Nullable<Int8>,
        expire_date -> Nullable<Timestamp>,
        days_before_trigger -> Int4,
        notify_trigger_date -> Nullable<Timestamp>,
        expire_date_ms -> Nullable<Int8>,
    }
}

table! {
    error_recover_record (id) {
        id -> Int8,
    }
}

table! {
    feedback (id) {
        id -> Int8,
        content -> Varchar,
        contact -> Nullable<Varchar>,
        feed_type -> Nullable<Int4>,
        created_time -> Int8,
        updated_time -> Int8,
        user_id -> Nullable<Int8>,
        app_version -> Nullable<Varchar>,
        app_id -> Int4,
        handle_status -> Nullable<Int4>,
        handle_result -> Nullable<Varchar>,
        handle_way -> Nullable<Int4>,
        email -> Nullable<Varchar>,
    }
}

table! {
    hot_search_words (id) {
        key_word -> Varchar,
        hit_count -> Int8,
        app_id -> Int4,
        id -> Int8,
    }
}

table! {
    iap_product (id) {
        id -> Int4,
        product_id -> Varchar,
        product_type -> Int4,
        online_status -> Int4,
        created_time -> Int8,
        updated_time -> Int8,
        product_title -> Varchar,
        description -> Varchar,
        price -> Varchar,
        raw_price -> Varchar,
        currency_code -> Nullable<Varchar>,
        app_id -> Int4,
        sort -> Varchar,
        deleted -> Nullable<Int4>,
    }
}

table! {
    notify_channel (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        created_time -> Nullable<Int8>,
        updated_time -> Nullable<Int8>,
        sort -> Nullable<Int4>,
        deleted -> Nullable<Int4>,
    }
}

table! {
    oauth (id) {
        id -> Int8,
        user_id -> Nullable<Int8>,
        oauth_type -> Int4,
        oauth_id -> Nullable<Varchar>,
        unionid -> Nullable<Varchar>,
        credential -> Nullable<Varchar>,
        salt -> Nullable<Varchar>,
    }
}

table! {
    pay_receipt_verify_record (id) {
        id -> Int8,
        environment -> Nullable<Varchar>,
        pay_status -> Nullable<Int4>,
        latest_receipt -> Nullable<Varchar>,
        transaction_id -> Varchar,
        created_time -> Int8,
        updated_time -> Int8,
        user_id -> Int8,
        receipt -> Varchar,
        app_id -> Int4,
        action_type -> Int4,
        is_retryable -> Int4,
        pending_renewal_info -> Nullable<Varchar>,
        latest_receipt_info -> Nullable<Varchar>,
        product_id -> Varchar,
        ip -> Nullable<Varchar>,
        device_id -> Varchar,
        unified_receipt -> Nullable<Varchar>,
        verify_status -> Int4,
    }
}

table! {
    pay_transaction_record (id) {
        id -> Int8,
        transaction_id -> Varchar,
        in_app_ownership_type -> Varchar,
        quantity -> Int4,
        origianl_transaction_id -> Varchar,
        subscription_group_identifier -> Varchar,
        purchase_date_pst -> Varchar,
        original_purchase_date -> Varchar,
        original_purchase_date_ms -> Int8,
        original_purchase_date_pst -> Varchar,
        original_application_version -> Nullable<Varchar>,
        is_in_intro_offer_period -> Int4,
        expires_date -> Varchar,
        is_trial_period -> Int4,
        expire_date_pst -> Varchar,
        expire_date_ms -> Int8,
        product_id -> Varchar,
        purchase_date -> Varchar,
        web_order_line_item_id -> Varchar,
        user_id -> Int8,
        app_id -> Int4,
        purchase_date_ms -> Int8,
        device_id -> Varchar,
        trigger_source -> Nullable<Int4>,
        action_type -> Nullable<Int4>,
        latest_receipt -> Varchar,
    }
}

table! {
    play_record (id) {
        id -> Int8,
        url -> Nullable<Varchar>,
        source -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
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
    search_history (id) {
        query_word -> Varchar,
        created_time -> Int8,
        updated_time -> Int8,
        app_id -> Int4,
        user_id -> Int8,
        device -> Int4,
        id -> Int8,
    }
}

table! {
    sub_relation (id) {
        id -> Int8,
        user_id -> Int8,
        sub_source_id -> Int8,
        created_time -> Int8,
        updated_time -> Int8,
        sub_status -> Nullable<Int4>,
    }
}

table! {
    tags (id) {
        id -> Int4,
        tag_name -> Varchar,
        created_time -> Int8,
        updated_time -> Int8,
        app_id -> Int4,
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
    user_extend (id) {
        id -> Int8,
        user_id -> Int8,
        field -> Varchar,
        value -> Varchar,
    }
}

table! {
    user_notify_binding (id) {
        id -> Int8,
        user_id -> Nullable<Int8>,
        notify_channel_id -> Nullable<Int4>,
        created_time -> Nullable<Int8>,
        updated_time -> Nullable<Int8>,
        deleted -> Nullable<Int4>,
        access_token -> Nullable<Varchar>,
        secret -> Nullable<Varchar>,
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
    actor,
    admin_users,
    app_log,
    apple_server_notification_record,
    apps,
    article,
    article_content,
    article_favorites,
    dashboard,
    domain,
    error_recover_record,
    feedback,
    hot_search_words,
    iap_product,
    notify_channel,
    oauth,
    pay_receipt_verify_record,
    pay_transaction_record,
    play_record,
    rss_sub_source,
    search_history,
    sub_relation,
    tags,
    trend,
    user_extend,
    user_notify_binding,
    users,
);
