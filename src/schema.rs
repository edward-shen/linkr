table! {
    links (id) {
        id -> Int4,
        owner -> Nullable<Int4>,
        origin -> Text,
        dest -> Text,
        is_private -> Bool,
        clicks -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password -> Text,
        email -> Text,
    }
}

joinable!(links -> users (owner));

allow_tables_to_appear_in_same_query!(
    links,
    users,
);
