table! {
    dict (id) {
        word -> Varchar,
        phonetic -> Varchar,
        definition -> Varchar,
        translation -> Varchar,
        pos -> Varchar,
        collins -> Nullable<Int4>,
        oxford -> Nullable<Int4>,
        tag -> Varchar,
        bnc -> Int4,
        frq -> Int4,
        exchange -> Varchar,
        detail -> Varchar,
        audio -> Varchar,
        id -> Int8,
        demo_sentence -> Int4,
        sentence_count -> Nullable<Int4>,
    }
}

table! {
    user_dict (id) {
        id -> Int8,
        word_id -> Int8,
        user_id -> Int8,
        created_time -> Int8,
        updated_time -> Int8,
        status -> Int4,
        word -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    dict,
    user_dict,
);
