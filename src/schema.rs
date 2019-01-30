table! {
    links (id) {
        id -> Int4,
        owner -> Nullable<Text>,
        origin -> Text,
        dest -> Text,
        creation_date -> Timestamp,
        last_used -> Nullable<Timestamp>,
        clicks -> Int4,
        expire_date -> Nullable<Timestamp>,
        expire_clicks -> Nullable<Int4>,
    }
}

table! {
    token_key_map (id) {
        id -> Int4,
        token -> Text,
        key -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    links,
    token_key_map,
);
