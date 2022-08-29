mod common;

use rocket::http::Status;

use oba_api::models::{Account, AccountForm};

use common::Setup;
use common::ACCOUNT_NUMBER;

const URL: &str = "/account";

#[test]
fn test_account_create() {
    // Setup test
    let client = &Setup::new().client;
    // Create accounts
    for account_index in 1..=ACCOUNT_NUMBER {
        let account_form = AccountForm::new(format!("account_name_{account_index}"));
        let response = client.post(URL).json(&account_form).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_json::<AccountForm>(), Some(account_form));
    }
}

#[test]
fn test_account_create_same_name() {
    // Setup test
    let client = &Setup::new().client;
    // Create account twice
    let account_form = AccountForm::new(String::from("account_name"));
    client.post(URL).json(&account_form).dispatch();
    let response = client.post(URL).json(&account_form).dispatch();
    assert_eq!(response.status(), Status::Conflict);
}

#[test]
fn test_account_list() {
    // Setup test
    let client = &Setup::new().client;
    // Create accounts
    let mut account_forms = Vec::with_capacity(ACCOUNT_NUMBER);
    for account_index in 1..=ACCOUNT_NUMBER {
        let account = AccountForm::new(format!("account_name_{account_index}"));
        client.post(URL).json(&account).dispatch();
        account_forms.push(account);
    }
    // Read accounts
    let response = client.get(format!("{}", URL)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    let accounts = response.into_json::<Vec<Account>>().unwrap();
    assert_eq!(accounts.len(), ACCOUNT_NUMBER);
    assert_eq!(
        accounts
            .iter()
            .map(Account::as_form)
            .collect::<Vec<AccountForm>>(),
        account_forms
    );
}

#[test]
fn test_account_read() {
    // Setup test
    let client = &Setup::new().client;
    // Create an account and get back id
    let account_form = AccountForm::new(String::from("account_name"));
    let account_request = client
        .post(URL)
        .json(&account_form)
        .dispatch()
        .into_json::<Account>()
        .unwrap();
    // Read
    let response = client
        .get(format!("{}/{}", URL, account_request.id()))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_json::<Account>(), Some(account_request));
}

#[test]
fn test_account_read_not_found() {
    // Setup test
    let client = &Setup::new().client;
    // Try reading
    let response = client.get(format!("{}/0", URL)).dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn test_account_delete() {
    // Setup test
    let client = &Setup::new().client;
    // Create an account and get back id
    let account_form = AccountForm::new(String::from("account_name"));
    let account_id = client
        .post(URL)
        .json(&account_form)
        .dispatch()
        .into_json::<Account>()
        .unwrap()
        .id();
    // Delete account
    client.delete(format!("{}/{}", URL, account_id)).dispatch();
    // Try reading
    let response = client.get(format!("{}/{}", URL, account_id)).dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn test_account_update() {
    // Setup test
    let client = &Setup::new().client;
    // Create an account and get back id
    let account_form = AccountForm::new(String::from("account_name"));
    let account_id = client
        .post(URL)
        .json(&account_form)
        .dispatch()
        .into_json::<Account>()
        .unwrap()
        .id();
    // Update account
    let new_account = AccountForm::new(String::from("new_name"));
    let response_update = client
        .put(format!("{}/{}", URL, account_id))
        .json(&new_account)
        .dispatch();
    assert_eq!(response_update.status(), Status::Ok);
    let returned_account = response_update.into_json::<Account>().unwrap();
    assert_eq!(returned_account.as_form(), new_account);
    assert_eq!(returned_account.id(), account_id);
    // Read
    let response_read = client.get(format!("{}/{}", URL, account_id)).dispatch();
    assert_eq!(response_read.status(), Status::Ok);
    assert_eq!(response_read.into_json::<AccountForm>(), Some(new_account));
}

#[test]
fn test_account_destroy() {
    let client = &Setup::new().client;
    assert_eq!(client.delete(URL).dispatch().status(), Status::Ok);
    assert_eq!(
        client.get(URL).dispatch().into_json::<Vec<Account>>(),
        Some(vec![])
    );
}
