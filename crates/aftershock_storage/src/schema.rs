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
