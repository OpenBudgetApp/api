table! {
    accounts (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    buckets (id) {
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
        bucket_id -> Nullable<Integer>,
    }
}

joinable!(transactions -> accounts (account_id));
joinable!(transactions -> buckets (bucket_id));

allow_tables_to_appear_in_same_query!(accounts, buckets, transactions,);
