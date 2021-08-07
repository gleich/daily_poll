table! {
    poll_options (id) {
        id -> Integer,
        question -> Varchar,
        option_name -> Varchar,
    }
}

table! {
    polls (question) {
        question -> Varchar,
        options -> Varchar,
        author -> Varchar,
        used -> Bool,
        add_options -> Bool,
        multiselect -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    poll_options,
    polls,
);
