table! {
    accounts (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    transactions (id) {
        id -> Integer,
        name -> Text,
        amount -> Float,
        date -> Timestamp,
        account_id -> Integer,
    }
}

joinable!(transactions -> accounts (account_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    transactions,
);
