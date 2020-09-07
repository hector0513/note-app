

table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        password -> Varchar,
        email -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    users,
);
