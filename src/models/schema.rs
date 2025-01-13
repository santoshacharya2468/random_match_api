// @generated automatically by Diesel CLI.

diesel::table! {
    auth_users (id) {
        id -> Uuid,
        email -> Nullable<Text>,
        username -> Text,
        password -> Nullable<Text>,
        provider -> Text,
        external_id -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    random_matches (id) {
        id -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        user_id -> Uuid,
        matched_user_id -> Nullable<Uuid>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    auth_users,
    random_matches,
);
