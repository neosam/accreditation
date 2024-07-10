// @generated automatically by Diesel CLI.

diesel::table! {
    attendant (id) {
        id -> Uuid,
        member_id -> Uuid,
        registered_at -> Timestamptz,
    }
}

diesel::table! {
    member (id) {
        id -> Uuid,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
        member_id -> Int4,
    }
}

diesel::joinable!(attendant -> member (member_id));

diesel::allow_tables_to_appear_in_same_query!(
    attendant,
    member,
);
