use rocket::serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

use super::schema::{accounts, transactions};

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
#[table_name = "transactions"]
pub struct Transaction {
    pub id: i32,
    pub name: String,
    pub amount: f32,
    pub date: NaiveDateTime,
    pub account_id: i32,
}

#[derive(Debug, PartialEq, Insertable, AsChangeset, Associations, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(belongs_to(Account, foreign_key = account_id))]
#[table_name = "transactions"]
pub struct TransactionForm {
    pub name: String,
    pub amount: f32,
    pub date: NaiveDateTime,
    pub account_id: i32,
}

impl TransactionForm {
    pub fn new(name: String, amount: f32, date: NaiveDateTime, account_id: i32) -> Self {
        TransactionForm { name, amount, date, account_id }
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
        }
    }
}
