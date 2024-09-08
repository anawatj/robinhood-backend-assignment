// @generated automatically by Diesel CLI.

diesel::table! {
    comments (id) {
        id -> Int4,
        job_id -> Int4,
        description -> Text,
        create_by -> Varchar,
        create_date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    jobs (id) {
        id -> Int4,
        title -> Varchar,
        description -> Text,
        status -> Varchar,
        create_by -> Varchar,
        create_date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 200]
        email -> Varchar,
        #[max_length = 8000]
        password -> Varchar,
        #[max_length = 200]
        first_name -> Varchar,
        #[max_length = 200]
        last_name -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    comments,
    jobs,
    users,
);
