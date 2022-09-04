use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};

use super::schema::{accounts, buckets, fills, transactions};

#[derive(Debug, PartialEq, Queryable, Identifiable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "accounts"]
pub struct Account {
    id: i32,
    name: String,
}

impl Account {
    pub fn display(&self) -> String {
        format!("[Account] <{}>.", self.name)
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn as_form(&self) -> AccountForm {
        AccountForm {
            name: self.name.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Insertable, AsChangeset, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "accounts"]
pub struct AccountForm {
    name: String,
}

impl AccountForm {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, PartialEq, Queryable, Identifiable, Associations, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Insertable, AsChangeset, Associations, Serialize, Deserialize)]
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

impl TransactionForm {
    pub fn new(
        name: String,
        amount: f32,
        date: NaiveDateTime,
        account_id: i32,
        bucket_id: Option<i32>,
    ) -> Self {
        TransactionForm {
            name,
            amount,
            date,
            account_id,
            bucket_id,
        }
    }
    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
}

impl Transaction {
    pub fn id(&self) -> i32 {
        self.id.clone()
    }

    pub fn as_form(&self) -> TransactionForm {
        TransactionForm {
            name: self.name.clone(),
            amount: self.amount,
            date: self.date.clone(),
            account_id: self.account_id,
            bucket_id: self.bucket_id,
        }
    }
}

#[derive(Debug, PartialEq, Queryable, Identifiable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "buckets"]
pub struct Bucket {
    id: i32,
    name: String,
}

impl Bucket {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn as_form(&self) -> BucketForm {
        BucketForm {
            name: self.name.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Insertable, AsChangeset, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "buckets"]
pub struct BucketForm {
    name: String,
}

impl BucketForm {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, PartialEq, Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(belongs_to(Bucket, foreign_key = bucket_id))]
#[table_name = "fills"]
pub struct Fill {
    id: i32,
    amount: f32,
    date: NaiveDateTime,
    bucket_id: i32,
}

#[derive(Debug, PartialEq, Insertable, AsChangeset, Associations, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(belongs_to(Bucket, foreign_key = bucket_id))]
#[table_name = "fills"]
pub struct FillForm {
    amount: f32,
    date: NaiveDateTime,
    bucket_id: i32,
}

impl FillForm {
    pub fn new(amount: f32, date: NaiveDateTime, bucket_id: i32) -> Self {
        Self {
            amount,
            date,
            bucket_id,
        }
    }

    pub fn with_amount(mut self, amount: f32) -> Self {
        self.amount = amount;
        self
    }
}

impl Fill {
    pub fn id(&self) -> i32 {
        self.id.clone()
    }

    pub fn as_form(&self) -> FillForm {
        FillForm {
            amount: self.amount,
            date: self.date.clone(),
            bucket_id: self.bucket_id,
        }
    }
}
