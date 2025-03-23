// @generated automatically by Diesel CLI.

diesel::table! {
    contents (id) {
        id -> Integer,
        kind -> Text,
        created_at -> BigInt,
        updated_at -> BigInt,
        title -> Text,
        body -> Text,
        published -> Bool,
        uid -> Text,
    }
}

diesel::table! {
    contents_tags (content_id, tag_id) {
        content_id -> Integer,
        tag_id -> Integer,
    }
}

diesel::table! {
    tags (id) {
        id -> Integer,
        tag -> Text,
    }
}

diesel::joinable!(contents_tags -> contents (content_id));
diesel::joinable!(contents_tags -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(contents, contents_tags, tags,);
