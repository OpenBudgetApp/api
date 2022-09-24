use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};

use super::schema::{accounts, buckets, fills, transactions};

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "accounts"]
pub struct Account {
    id: i32,
    name: String,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "accounts"]
pub struct AccountForm {
    name: String,
}

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(belongs_to(Account, foreign_key = account_id))]
#[diesel(belongs_to(Bucket, foreign_key = bucket_id))]
#[table_name = "transactions"]
pub struct Transaction {
    id: i32,
    name: String,
    amount: f32,
    date: NaiveDateTime,
    account_id: i32,
    bucket_id: Option<i32>,
}

#[derive(Insertable, AsChangeset, Associations, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(belongs_to(Account, foreign_key = account_id))]
#[diesel(belongs_to(Bucket, foreign_key = bucket_id))]
#[table_name = "transactions"]
pub struct TransactionForm {
    name: String,
    amount: f32,
    date: NaiveDateTime,
    account_id: i32,
    bucket_id: Option<i32>,
}

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "buckets"]
pub struct Bucket {
    id: i32,
    name: String,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "buckets"]
pub struct BucketForm {
    name: String,
}

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(belongs_to(Bucket, foreign_key = bucket_id))]
#[table_name = "fills"]
pub struct Fill {
    id: i32,
    amount: f32,
    date: NaiveDateTime,
    bucket_id: i32,
}

#[derive(Insertable, AsChangeset, Associations, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(belongs_to(Bucket, foreign_key = bucket_id))]
#[table_name = "fills"]
pub struct FillForm {
    amount: f32,
    date: NaiveDateTime,
    bucket_id: i32,
}
