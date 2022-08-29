use oba_api::models::{Account, AccountForm};
use rocket::local::blocking::Client;

use oba_api::api::{account, transaction};
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
                .attach(transaction::stage()),
        )
        .unwrap();
        client.delete(URL_TRANSACTION).dispatch().status();
        client.delete(URL_ACCOUNT).dispatch().status();
        Self { client }
    }

    #[allow(dead_code)]
    pub fn create_account(&self) -> i32 {
        self.client
            .post(URL_ACCOUNT)
            .json(&AccountForm::new(format!("account_name")))
            .dispatch()
            .into_json::<Account>()
            .unwrap()
            .id()
    }
}

impl Drop for Setup {
    fn drop(&mut self) {
        self.client.delete(URL_TRANSACTION).dispatch().status();
        self.client.delete(URL_ACCOUNT).dispatch().status();
    }
}

pub const URL_TRANSACTION: &str = "/transaction";
pub const URL_ACCOUNT: &str = "/account";
#[allow(dead_code)]
pub const TRANSACTION_NUMBER: usize = 3;
#[allow(dead_code)]
pub const ACCOUNT_NUMBER: usize = 3;
