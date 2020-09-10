table! {
    heroes (id) {
        id -> Int4,
        fantasy_name -> Varchar,
        real_name -> Nullable<Varchar>,
        spotted_photo -> Text,
        strength_level -> Int4,
    }
}

table! {
    notes (id) {
        id -> Int4,
        title -> Varchar,
        description -> Varchar,
        created -> Timestamp,
    }
}

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
    heroes,
    notes,
    users,
);
