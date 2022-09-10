use chrono::{Local, NaiveDateTime};
use rocket::local::blocking::Client;
use rocket::serde::{Deserialize, Serialize};

use oba_api::api::{account, bucket, fill, transaction};
use oba_api::DbConnection;

pub struct Setup {
    pub client: Client,
}

impl Setup {
    pub fn new() -> Self {
        let client = Client::tracked(
            rocket::build()
                .attach(DbConnection::fairing())
                .attach(account::stage())
                .attach(transaction::stage())
                .attach(bucket::stage())
                .attach(fill::stage()),
        )
        .unwrap();
        client.delete(URL_TRANSACTION).dispatch().status();
        client.delete(URL_FILL).dispatch().status();
        client.delete(URL_BUCKET).dispatch().status();
        client.delete(URL_ACCOUNT).dispatch().status();
        Self { client }
    }

    #[allow(dead_code)]
    pub fn create_account(&self) -> i32 {
        self.client
            .post(URL_ACCOUNT)
            .json(&Account::new(format!(
                "account_{}",
                Local::now().to_rfc3339()
            )))
            .dispatch()
            .into_json::<Account>()
            .unwrap()
            .id
            .unwrap()
    }

    #[allow(dead_code)]
    pub fn create_bucket(&self) -> i32 {
        self.client
            .post(URL_BUCKET)
            .json(&Bucket::new(format!(
                "bucket_{}",
                Local::now().to_rfc3339()
            )))
            .dispatch()
            .into_json::<Bucket>()
            .unwrap()
            .id
            .unwrap()
    }
}

impl Drop for Setup {
    fn drop(&mut self) {
        self.client.delete(URL_TRANSACTION).dispatch();
        self.client.delete(URL_FILL).dispatch();
        self.client.delete(URL_BUCKET).dispatch();
        self.client.delete(URL_ACCOUNT).dispatch();
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Account {
    #[serde(skip_serializing)]
    pub id: Option<i32>,
    pub name: String,
}

impl Account {
    pub fn new(name: String) -> Self {
        Self { id: None, name }
    }
}

impl PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Transaction {
    #[serde(skip_serializing)]
    pub id: Option<i32>,
    pub name: String,
    pub amount: f32,
    pub date: NaiveDateTime,
    pub account_id: i32,
    pub bucket_id: Option<i32>,
}

impl Transaction {
    #[allow(dead_code)]
    pub fn new(
        name: String,
        amount: f32,
        date: NaiveDateTime,
        account_id: i32,
        bucket_id: Option<i32>,
    ) -> Self {
        Self {
            id: None,
            name,
            amount,
            date,
            account_id,
            bucket_id,
        }
    }

    #[allow(dead_code)]
    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
}

impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        (self.name == other.name)
            && (self.amount == other.amount)
            && (self.date == other.date)
            && (self.account_id == other.account_id)
            && (self.bucket_id == other.bucket_id)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Bucket {
    #[serde(skip_serializing)]
    pub id: Option<i32>,
    pub name: String,
}

impl Bucket {
    pub fn new(name: String) -> Self {
        Self { id: None, name }
    }
}

impl PartialEq for Bucket {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Fill {
    #[serde(skip_serializing)]
    pub id: Option<i32>,
    pub amount: f32,
    pub date: NaiveDateTime,
    pub bucket_id: i32,
}

impl Fill {
    #[allow(dead_code)]
    pub fn new(amount: f32, date: NaiveDateTime, bucket_id: i32) -> Self {
        Self {
            id: None,
            amount,
            date,
            bucket_id,
        }
    }

    #[allow(dead_code)]
    pub fn with_amount(mut self, amount: f32) -> Self {
        self.amount = amount;
        self
    }
}

impl PartialEq for Fill {
    fn eq(&self, other: &Self) -> bool {
        (self.amount == other.amount)
            && (self.date == other.date)
            && (self.bucket_id == other.bucket_id)
    }
}

pub const URL_TRANSACTION: &str = "/transaction";
pub const URL_ACCOUNT: &str = "/account";
pub const URL_BUCKET: &str = "/bucket";
pub const URL_FILL: &str = "/fill";
#[allow(dead_code)]
pub const TRANSACTION_NUMBER: usize = 3;
#[allow(dead_code)]
pub const ACCOUNT_NUMBER: usize = 3;
#[allow(dead_code)]
pub const BUCKET_NUMBER: usize = 3;
#[allow(dead_code)]
pub const FILL_NUMBER: usize = 3;
