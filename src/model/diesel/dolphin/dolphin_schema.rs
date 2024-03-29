table! {
    admin_users (id) {
        id -> Int8,
        nickname -> Varchar,
        avatar_url -> Varchar,
        phone -> Varchar,
        updated_time -> Int8,
        created_time -> Int8,
        salt -> Varchar,
        pwd -> Varchar,
        sex -> Nullable<Int4>,
        level_type -> Nullable<Varchar>,
        phone_region -> Varchar,
        country_code -> Int4,
        user_status -> Int4,
        user_name -> Varchar,
        org_id -> Int4,
    }
}

table! {
    app_repo (id) {
        id -> Int8,
        app_name -> Varchar,
        release_datetime -> Int8,
        created_time -> Int8,
        updated_time -> Int8,
        linux_run -> Int4,
        windows_run -> Int4,
        macos_run -> Int4,
        intro -> Varchar,
        license -> Int4,
        official_url -> Nullable<Varchar>,
        source_url -> Nullable<Varchar>,
        author -> Nullable<Varchar>,
    }
}

table! {
    apps (id) {
        id -> Int4,
        app_name -> Varchar,
        remark -> Varchar,
        created_time -> Int8,
        updated_time -> Int8,
        user_count -> Int4,
        online_status -> Int4,
        online_time -> Nullable<Int8>,
        app_abbr -> Varchar,
        app_id -> Varchar,
        app_tag -> Nullable<Varchar>,
        auth_mode -> Int2,
        product_id -> Int4,
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
        permanent_store -> Int2,
    }
}

table! {
    article_content (id) {
        id -> Int8,
        article_id -> Int8,
        content -> Varchar,
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
        user_count -> Int4,
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
        monitor_status -> Int4,
        user_id -> Nullable<Int8>,
        expire_date -> Nullable<Timestamp>,
        days_before_trigger -> Int4,
        notify_trigger_date -> Nullable<Timestamp>,
        expire_date_ms -> Nullable<Int8>,
    }
}

table! {
    iap_product (id) {
        id -> Int8,
        product_id -> Int4,
        product_type -> Int4,
        online_status -> Int4,
        created_time -> Int8,
        updated_time -> Int8,
        product_title -> Varchar,
        description -> Varchar,
        price -> Numeric,
        raw_price -> Numeric,
        currency_code -> Nullable<Varchar>,
        app_id -> Varchar,
        sort -> Int4,
        deleted -> Int4,
        amount -> Int4,
        period -> Int4,
    }
}

table! {
    interview (id) {
        id -> Int8,
        city -> Varchar,
        company -> Varchar,
        created_time -> Int8,
        updated_time -> Int8,
        address -> Varchar,
        status -> Int4,
        interview_time -> Nullable<Int8>,
        info_source -> Int4,
        salary_range -> Nullable<Varchar>,
        apply_time -> Nullable<Int8>,
        apply_job -> Nullable<Varchar>,
        user_id -> Nullable<Int8>,
        job_link -> Nullable<Varchar>,
    }
}

table! {
    menu_resource (id) {
        id -> Int4,
        name -> Varchar,
        res_type -> Int4,
        created_time -> Int8,
        updated_time -> Int8,
        remark -> Nullable<Varchar>,
        path -> Varchar,
        parent_id -> Int4,
        component -> Nullable<Varchar>,
        sort -> Int4,
        name_zh -> Varchar,
        tree_id_path -> Varchar,
        code -> Varchar,
    }
}

table! {
    org (id) {
        id -> Int4,
        parent_id -> Int4,
        created_time -> Int8,
        updated_time -> Int8,
        org_name -> Varchar,
        sort -> Int4,
        tree_id_path -> Varchar,
    }
}

table! {
    products (id) {
        id -> Int4,
        product_name -> Varchar,
        remark -> Nullable<Varchar>,
        created_time -> Int8,
        updated_time -> Int8,
        user_count -> Int4,
        online_status -> Int4,
        online_time -> Nullable<Int8>,
        product_abbr -> Varchar,
        product_id -> Int4,
        product_tag -> Nullable<Varchar>,
    }
}

table! {
    role (id) {
        id -> Int4,
        name -> Varchar,
        status -> Int4,
        updated_time -> Int8,
        created_time -> Int8,
        remark -> Varchar,
        is_deleted -> Int4,
    }
}

table! {
    role_permission (id) {
        id -> Int8,
        role_id -> Int4,
        permission_id -> Int4,
        created_time -> Int8,
        updated_time -> Int8,
        permission_type -> Int4,
    }
}

table! {
    rss_sub_source (id) {
        id -> Int8,
        sub_url -> Varchar,
        created_time -> Int8,
        updated_time -> Int8,
        sub_status -> Int4,
        rss_type -> Varchar,
        standard_type -> Varchar,
        standard_version -> Varchar,
        cron -> Varchar,
        trigger_count -> Int4,
        next_trigger_time -> Nullable<Timestamp>,
        sub_name -> Varchar,
        last_trigger_time -> Nullable<Timestamptz>,
        source_url -> Nullable<Varchar>,
        sub_type -> Nullable<Varchar>,
        intro -> Nullable<Varchar>,
        remark -> Nullable<Varchar>,
        title_hash -> Nullable<Varchar>,
        failed_count -> Int4,
        lang -> Nullable<Varchar>,
        frequency_month -> Nullable<Int4>,
        reputation -> Nullable<Int8>,
        rep_latest_refresh_time -> Nullable<Int8>,
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
        tags -> Jsonb,
        article_count -> Int8,
        article_count_latest_refresh_time -> Int8,
        comment_rss -> Int4,
        part_output -> Int4,
    }
}

table! {
    tags (id) {
        id -> Int4,
        tag_name -> Varchar,
        created_time -> Int8,
        updated_time -> Int8,
        app_id -> Int4,
        remark -> Nullable<Varchar>,
        group -> Int4,
        tag_type -> Varchar,
        code -> Varchar,
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
    user_devices (id) {
        id -> Int8,
        device_id -> Varchar,
        device_name -> Nullable<Varchar>,
        user_id -> Int8,
        created_time -> Int8,
        updated_time -> Int8,
    }
}

table! {
    user_role (id) {
        id -> Int4,
        user_id -> Int8,
        role_id -> Int4,
        created_time -> Int8,
        updated_time -> Int8,
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
        app_id -> Varchar,
        register_time -> Int8,
        apple_iap_product_id -> Nullable<Varchar>,
        auto_renew_product_expire_time_ms -> Nullable<Int8>,
        is_guest -> Int4,
        product_id -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    admin_users,
    app_repo,
    apps,
    article,
    article_content,
    article_favorites,
    dashboard,
    domain,
    iap_product,
    interview,
    menu_resource,
    org,
    products,
    role,
    role_permission,
    rss_sub_source,
    tags,
    trend,
    user_devices,
    user_role,
    users,
);
