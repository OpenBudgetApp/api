mod common;

use chrono::{Duration, NaiveDateTime};
use rocket::http::Status;

use oba_api::models::{Fill, FillForm};

use common::Setup;
use common::{FILL_NUMBER, URL_BUCKET, URL_FILL};

fn default_fill(bucket_id: i32) -> FillForm {
    let date = NaiveDateTime::parse_from_str("2022-07-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    FillForm::new(133.7, date, bucket_id)
}

#[test]
fn test_fill_create() {
    // Setup test
    let setup = Setup::new();
    let client = &setup.client;
    let bucket_id = setup.create_bucket();
    // Create fills
    for _ in 1..=FILL_NUMBER {
        let fill_form = default_fill(bucket_id);
        let response = client.post(URL_FILL).json(&fill_form).dispatch();
        assert_eq!(response.status(), Status::Created);
        assert_eq!(response.into_json::<FillForm>(), Some(fill_form));
    }
}

#[test]
fn test_fill_create_invalid_bucket_id() {
    // Setup test
    let setup = Setup::new();
    let client = &setup.client;
    let bucket_id = setup.create_bucket();
    // Create fills
    let fill_form = default_fill(bucket_id + 1);
    let response = client.post(URL_FILL).json(&fill_form).dispatch();
    assert_eq!(response.status(), Status::Conflict);
}

#[test]
fn test_fill_list() {
    // Setup test
    let setup = Setup::new();
    let client = &setup.client;
    let bucket_id = setup.create_bucket();
    // Create fills
    let mut fill_forms = Vec::with_capacity(FILL_NUMBER);
    for _ in 1..=FILL_NUMBER {
        let fill_form = default_fill(bucket_id);
        client.post(URL_FILL).json(&fill_form).dispatch();
        fill_forms.push(fill_form);
    }
    // Read fills
    let response = client.get(format!("{}", URL_FILL)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    let fills = response.into_json::<Vec<Fill>>().unwrap();
    assert_eq!(fills.len(), FILL_NUMBER);
    assert_eq!(
        fills.iter().map(Fill::as_form).collect::<Vec<FillForm>>(),
        fill_forms
    );
}

#[test]
fn test_fill_read() {
    // Setup test
    let setup = Setup::new();
    let client = &setup.client;
    let bucket_id = setup.create_bucket();
    // Create a fill and get back id
    let fill_form = default_fill(bucket_id);
    let fill_request = client
        .post(URL_FILL)
        .json(&fill_form)
        .dispatch()
        .into_json::<Fill>()
        .unwrap();
    // Read
    let response = client
        .get(format!("{}/{}", URL_FILL, fill_request.id()))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_json::<Fill>(), Some(fill_request));
}

#[test]
fn test_fill_read_not_found() {
    // Setup test
    let client = &Setup::new().client;
    // Try reading
    let response = client.get(format!("{}/0", URL_FILL)).dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn test_fill_delete() {
    // Setup test
    let setup = Setup::new();
    let client = &setup.client;
    let bucket_id = setup.create_bucket();
    // Create a fill and get back id
    let fill_form = default_fill(bucket_id);
    let fill_id = client
        .post(URL_FILL)
        .json(&fill_form)
        .dispatch()
        .into_json::<Fill>()
        .unwrap()
        .id();
    // Delete fill
    client
        .delete(format!("{}/{}", URL_FILL, fill_id))
        .dispatch();
    // Try reading
    let response = client.get(format!("{}/{}", URL_FILL, fill_id)).dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn test_fill_delete_bucket_not_empty() {
    // Setup test
    let setup = Setup::new();
    let client = &setup.client;
    let bucket_id = setup.create_bucket();
    // Create a fill
    let fill_form = default_fill(bucket_id);
    client
        .post(URL_FILL)
        .json(&fill_form)
        .dispatch()
        .into_json::<Fill>()
        .unwrap();
    // Try deleting bucket
    let response = client
        .delete(format!("{}/{}", URL_BUCKET, bucket_id))
        .dispatch();
    assert_eq!(response.status(), Status::Conflict);
}

#[test]
fn test_fill_update() {
    // Setup test
    let setup = Setup::new();
    let client = &setup.client;
    let bucket_id = setup.create_bucket();
    // Create a fill
    let fill_form = default_fill(bucket_id);
    client.post(URL_FILL).json(&fill_form).dispatch();
    let fill_id = client
        .post(URL_FILL)
        .json(&fill_form)
        .dispatch()
        .into_json::<Fill>()
        .unwrap()
        .id();
    client.post(URL_FILL).json(&fill_form).dispatch();
    // Update fill
    let new_fill = fill_form.with_amount(342.4);
    let response_update = client
        .put(format!("{}/{}", URL_FILL, fill_id))
        .json(&new_fill)
        .dispatch();
    assert_eq!(response_update.status(), Status::Ok);
    let returned_fill = response_update.into_json::<Fill>().unwrap();
    assert_eq!(returned_fill.as_form(), new_fill);
    assert_eq!(returned_fill.id(), fill_id);
    // Read
    let response_read = client.get(format!("{}/{}", URL_FILL, fill_id)).dispatch();
    assert_eq!(response_read.status(), Status::Ok);
    assert_eq!(response_read.into_json::<FillForm>(), Some(new_fill));
}

#[test]
fn test_fill_destroy() {
    // Setup test
    let setup = Setup::new();
    let client = &setup.client;
    let bucket_id = setup.create_bucket();
    // Create a fill
    let fill_form = default_fill(bucket_id);
    client.post(URL_FILL).json(&fill_form).dispatch();
    // Delete all fills
    assert_eq!(client.delete(URL_FILL).dispatch().status(), Status::Ok);
    // Check the list is empty
    assert_eq!(
        client.get(URL_FILL).dispatch().into_json::<Vec<Fill>>(),
        Some(vec![])
    );
}

#[test]
fn test_fill_per_bucket() {
    // Setup test
    let setup = Setup::new();
    let client = &setup.client;
    // Create 3 buckets
    setup.create_bucket();
    let bucket_id = setup.create_bucket();
    setup.create_bucket();
    // Create a few fills for the second bucket
    let date = NaiveDateTime::parse_from_str("2022-06-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let fills = [
        FillForm::new(10.1, date, bucket_id),
        FillForm::new(20.2, date, bucket_id),
        FillForm::new(30.3, date, bucket_id),
        FillForm::new(40.4, date, bucket_id),
    ];
    for fill in &fills {
        client.post(URL_FILL).json(fill).dispatch();
    }
    // List fills for this bucket
    let response = client
        .get(format!("{}/{}/fills", URL_BUCKET, bucket_id,))
        .dispatch();
    // Check that the list matches
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_json::<Vec<FillForm>>().unwrap(), fills);
}

#[test]
fn test_fill_per_month() {
    // Setup test
    let setup = Setup::new();
    let client = &setup.client;
    let bucket_id = setup.create_bucket();
    // Create a few fills
    let date = NaiveDateTime::parse_from_str("2022-06-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let fills = [
        FillForm::new(10.1, date, bucket_id),
        FillForm::new(20.2, date + Duration::days(31), bucket_id),
        FillForm::new(40.4, date + Duration::days(50), bucket_id),
        FillForm::new(30.3, date + Duration::days(65), bucket_id),
    ];
    for fill in &fills {
        client.post(URL_FILL).json(fill).dispatch();
    }
    // List fills for this bucket for July
    let response = client
        .get(format!("{}/{}/fills/2022/07", URL_BUCKET, bucket_id,))
        .dispatch();
    // Check that the list matches
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_json::<Vec<FillForm>>().unwrap(), fills[1..=2]);
}
