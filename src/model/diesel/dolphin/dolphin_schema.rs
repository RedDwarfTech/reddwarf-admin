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


allow_tables_to_appear_in_same_query!(
    admin_users,
);
