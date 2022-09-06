mod common;

use chrono::{Duration, NaiveDateTime};
use rocket::http::Status;
use std::iter::zip;

use oba_api::models::{Transaction, TransactionForm};

use common::Setup;
use common::{TRANSACTION_NUMBER, URL_ACCOUNT, URL_BUCKET, URL_TRANSACTION};

fn default_transaction(account_id: i32) -> TransactionForm {
    let date = NaiveDateTime::parse_from_str("2022-07-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    TransactionForm::new(
        String::from("transaction_name"),
        133.7,
        date,
        account_id,
        None,
    )
}

#[test]
fn test_transaction_create() {
    // Setup test
    let setup = Setup::new();
    let client = &setup.client;
    let account_id = setup.create_account();
    // Create transactions
    for _ in 1..=TRANSACTION_NUMBER {
        let transaction_form = default_transaction(account_id);
        let response = client
            .post(URL_TRANSACTION)
            .json(&transaction_form)
            .dispatch();
        assert_eq!(response.status(), Status::Created);
        assert_eq!(
            response.into_json::<TransactionForm>(),
            Some(transaction_form)
        );
    }
}

#[test]
fn test_transaction_create_invalid_account_id() {
    // Setup test
    let setup = Setup::new();
    let client = &setup.client;
    let account_id = setup.create_account();
    // Create transactions
    let transaction_form = default_transaction(account_id + 1);
    let response = client
        .post(URL_TRANSACTION)
        .json(&transaction_form)
        .dispatch();
    assert_eq!(response.status(), Status::Conflict);
}

#[test]
fn test_transaction_list() {
    // Setup test
    let setup = Setup::new();
    let client = &setup.client;
    let account_id = setup.create_account();
    // Create transactions
    let mut transaction_forms = Vec::with_capacity(TRANSACTION_NUMBER);
    for _ in 1..=TRANSACTION_NUMBER {
        let transaction_form = default_transaction(account_id);
        client
            .post(URL_TRANSACTION)
            .json(&transaction_form)
            .dispatch();
        transaction_forms.push(transaction_form);
    }
    // Read transactions
    let response = client.get(format!("{}", URL_TRANSACTION)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    let transactions = response.into_json::<Vec<Transaction>>().unwrap();
    assert_eq!(transactions.len(), TRANSACTION_NUMBER);
    assert_eq!(
        transactions
            .iter()
            .map(Transaction::as_form)
            .collect::<Vec<TransactionForm>>(),
        transaction_forms
    );
}

#[test]
fn test_transaction_read() {
    // Setup test
    let setup = Setup::new();
    let client = &setup.client;
    let account_id = setup.create_account();
    // Create a transaction and get back id
    let transaction_form = default_transaction(account_id);
    let transaction_request = client
        .post(URL_TRANSACTION)
        .json(&transaction_form)
        .dispatch()
        .into_json::<Transaction>()
        .unwrap();
    // Read
    let response = client
        .get(format!("{}/{}", URL_TRANSACTION, transaction_request.id()))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.into_json::<Transaction>(),
        Some(transaction_request)
    );
}

#[test]
fn test_transaction_read_not_found() {
    // Setup test
    let client = &Setup::new().client;
    // Try reading
    let response = client.get(format!("{}/0", URL_TRANSACTION)).dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn test_transaction_delete() {
    // Setup test
    let setup = Setup::new();
    let client = &setup.client;
    let account_id = setup.create_account();
    // Create a transaction and get back id
    let transaction_form = default_transaction(account_id);
    let transaction_id = client
        .post(URL_TRANSACTION)
        .json(&transaction_form)
        .dispatch()
        .into_json::<Transaction>()
        .unwrap()
        .id();
    // Delete transaction
    client
        .delete(format!("{}/{}", URL_TRANSACTION, transaction_id))
        .dispatch();
    // Try reading
    let response = client
        .get(format!("{}/{}", URL_TRANSACTION, transaction_id))
        .dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn test_transaction_delete_account_not_empty() {
    // Setup test
    let setup = Setup::new();
    let client = &setup.client;
    let account_id = setup.create_account();
    // Create a transaction
    let transaction_form = default_transaction(account_id);
    client
        .post(URL_TRANSACTION)
        .json(&transaction_form)
        .dispatch()
        .into_json::<Transaction>()
        .unwrap();
    // Try deleting account
    let response = client
        .delete(format!("{}/{}", URL_ACCOUNT, account_id))
        .dispatch();
    assert_eq!(response.status(), Status::Conflict);
}

#[test]
fn test_transaction_update() {
    // Setup test
    let setup = Setup::new();
    let client = &setup.client;
    let account_id = setup.create_account();
    // Create a transaction
    let transaction_form = default_transaction(account_id);
    client
        .post(URL_TRANSACTION)
        .json(&transaction_form)
        .dispatch();
    let transaction_id = client
        .post(URL_TRANSACTION)
        .json(&transaction_form)
        .dispatch()
        .into_json::<Transaction>()
        .unwrap()
        .id();
    client
        .post(URL_TRANSACTION)
        .json(&transaction_form)
        .dispatch();
    // Update transaction
    let new_transaction = transaction_form.with_name(String::from("new_transaction_name"));
    let response_update = client
        .put(format!("{}/{}", URL_TRANSACTION, transaction_id))
        .json(&new_transaction)
        .dispatch();
    assert_eq!(response_update.status(), Status::Ok);
    let returned_transaction = response_update.into_json::<Transaction>().unwrap();
    assert_eq!(returned_transaction.as_form(), new_transaction);
    assert_eq!(returned_transaction.id(), transaction_id);
    // Read
    let response_read = client
        .get(format!("{}/{}", URL_TRANSACTION, transaction_id))
        .dispatch();
    assert_eq!(response_read.status(), Status::Ok);
    assert_eq!(
        response_read.into_json::<TransactionForm>(),
        Some(new_transaction)
    );
}

#[test]
fn test_transaction_destroy() {
    // Setup test
    let setup = Setup::new();
    let client = &setup.client;
    let account_id = setup.create_account();
    // Create a transaction
    let transaction_form = default_transaction(account_id);
    client
        .post(URL_TRANSACTION)
        .json(&transaction_form)
        .dispatch();
    // Delete all transactions
    assert_eq!(
        client.delete(URL_TRANSACTION).dispatch().status(),
        Status::Ok
    );
    // Check the list is empty
    assert_eq!(
        client
            .get(URL_TRANSACTION)
            .dispatch()
            .into_json::<Vec<Transaction>>(),
        Some(vec![])
    );
}

#[test]
fn test_transaction_per_account() {
    // Setup test
    let setup = Setup::new();
    let client = &setup.client;
    // Create 3 accounts
    setup.create_account();
    let account_id = setup.create_account();
    setup.create_account();
    // Create a few transactions for the second account
    let date = NaiveDateTime::parse_from_str("2022-06-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let transactions = [
        TransactionForm::new(String::from("t1"), 10.1, date, account_id, None),
        TransactionForm::new(String::from("t2"), 20.2, date, account_id, None),
        TransactionForm::new(String::from("t3"), 30.3, date, account_id, None),
        TransactionForm::new(String::from("t4"), 40.4, date, account_id, None),
    ];
    for transaction in &transactions {
        client.post(URL_TRANSACTION).json(transaction).dispatch();
    }
    // List transactions for this account
    let response = client
        .get(format!("{}/{}/transactions", URL_ACCOUNT, account_id,))
        .dispatch();
    // Check that the list matches
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.into_json::<Vec<TransactionForm>>().unwrap(),
        transactions
    );
}

#[test]
fn test_transaction_per_month() {
    // Setup test
    let setup = Setup::new();
    let client = &setup.client;
    let account_id = setup.create_account();
    // Create a few transactions
    let date = NaiveDateTime::parse_from_str("2022-06-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let transactions = [
        TransactionForm::new(String::from("t1_june"), 10.1, date, account_id, None),
        TransactionForm::new(
            String::from("t2_july"),
            20.2,
            date + Duration::days(31),
            account_id,
            None,
        ),
        TransactionForm::new(
            String::from("t4_july"),
            40.4,
            date + Duration::days(50),
            account_id,
            None,
        ),
        TransactionForm::new(
            String::from("t3_august"),
            30.3,
            date + Duration::days(65),
            account_id,
            None,
        ),
    ];
    for transaction in &transactions {
        client.post(URL_TRANSACTION).json(transaction).dispatch();
    }
    // List transactions for this account for July
    let response = client
        .get(format!(
            "{}/{}/transactions/2022/07",
            URL_ACCOUNT, account_id,
        ))
        .dispatch();
    // Check that the list matches
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.into_json::<Vec<TransactionForm>>().unwrap(),
        transactions[1..=2]
    );
}

#[test]
fn test_transaction_per_bucket() {
    // Setup test
    let setup = Setup::new();
    let client = &setup.client;
    let account_id = setup.create_account();
    // Create 3 accounts
    let bucket_1_id = setup.create_bucket();
    let bucket_2_id = setup.create_bucket();
    let bucket_3_id = setup.create_bucket();
    // Create a few transactions for the second account
    let date = NaiveDateTime::parse_from_str("2022-06-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let transactions = [
        TransactionForm::new(
            String::from("t1"),
            10.1,
            date,
            account_id,
            Some(bucket_1_id),
        ),
        TransactionForm::new(
            String::from("t2"),
            20.2,
            date,
            account_id,
            Some(bucket_2_id),
        ),
        TransactionForm::new(
            String::from("t3"),
            30.3,
            date,
            account_id,
            Some(bucket_3_id),
        ),
        TransactionForm::new(String::from("t4"), 40.4, date, account_id, None),
    ];
    for transaction in &transactions {
        client.post(URL_TRANSACTION).json(transaction).dispatch();
    }
    for (bucket_id, transaction) in zip([bucket_1_id, bucket_2_id, bucket_3_id], &transactions) {
        let response = client
            .get(format!("{}/{}/transactions", URL_BUCKET, bucket_id))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_json::<Vec<TransactionForm>>().unwrap()[0],
            *transaction
        );
    }
}
