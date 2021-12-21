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

allow_tables_to_appear_in_same_query!(
    admin_users,
    article,
    article_content,
    dashboard,
);
