// @generated automatically by Diesel CLI.

diesel::table! {
    messages (id) {
        id -> Int4,
        username -> Text,
        body -> Text,
        time -> Timestamp,
    }
}
