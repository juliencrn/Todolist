table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        published -> Integer,
    }
}

table! {
    tasks (id) {
        id -> Integer,
        text -> Text,
        completed -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    tasks,
);
