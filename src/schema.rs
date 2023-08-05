// @generated automatically by Diesel CLI.

diesel::table! {
    _sqlx_migrations (version) {
        version -> Nullable<BigInt>,
        description -> Text,
        installed_on -> Timestamp,
        success -> Bool,
        checksum -> Binary,
        execution_time -> BigInt,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
        password -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    _sqlx_migrations,
    users,
);
