mod common;

use rocket::http::Status;

use oba_api::models::{Bucket, BucketForm};

use common::{Setup, BUCKET_NUMBER, URL_BUCKET};

#[test]
fn test_bucket_create() {
    // Setup test
    let client = &Setup::new().client;
    // Create buckets
    for index in 1..=BUCKET_NUMBER {
        let bucket_form = BucketForm::new(format!("bucket_name_{index}"));
        let response = client.post(URL_BUCKET).json(&bucket_form).dispatch();
        assert_eq!(response.status(), Status::Created);
        assert_eq!(response.into_json::<BucketForm>(), Some(bucket_form));
    }
}

#[test]
fn test_bucket_create_same_name() {
    // Setup test
    let client = &Setup::new().client;
    // Create bucket twice
    let bucket_form = BucketForm::new(String::from("bucket_name"));
    client.post(URL_BUCKET).json(&bucket_form).dispatch();
    let response = client.post(URL_BUCKET).json(&bucket_form).dispatch();
    assert_eq!(response.status(), Status::Conflict);
}

#[test]
fn test_bucket_list() {
    // Setup test
    let client = &Setup::new().client;
    // Create buckets
    let mut bucket_forms = Vec::with_capacity(BUCKET_NUMBER);
    for bucket_index in 1..=BUCKET_NUMBER {
        let bucket = BucketForm::new(format!("bucket_name_{bucket_index}"));
        client.post(URL_BUCKET).json(&bucket).dispatch();
        bucket_forms.push(bucket);
    }
    // Read buckets
    let response = client.get(format!("{}", URL_BUCKET)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    let buckets = response.into_json::<Vec<Bucket>>().unwrap();
    assert_eq!(buckets.len(), BUCKET_NUMBER);
    assert_eq!(
        buckets
            .iter()
            .map(Bucket::as_form)
            .collect::<Vec<BucketForm>>(),
        bucket_forms
    );
}

#[test]
fn test_bucket_read() {
    // Setup test
    let client = &Setup::new().client;
    // Create an bucket and get back id
    let bucket_form = BucketForm::new(String::from("bucket_name"));
    let bucket_request = client
        .post(URL_BUCKET)
        .json(&bucket_form)
        .dispatch()
        .into_json::<Bucket>()
        .unwrap();
    // Read
    let response = client
        .get(format!("{}/{}", URL_BUCKET, bucket_request.id()))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_json::<Bucket>(), Some(bucket_request));
}

#[test]
fn test_bucket_read_not_found() {
    // Setup test
    let client = &Setup::new().client;
    // Try reading
    let response = client.get(format!("{}/0", URL_BUCKET)).dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn test_bucket_delete() {
    // Setup test
    let client = &Setup::new().client;
    // Create an bucket and get back id
    let bucket_form = BucketForm::new(String::from("bucket_name"));
    let bucket_id = client
        .post(URL_BUCKET)
        .json(&bucket_form)
        .dispatch()
        .into_json::<Bucket>()
        .unwrap()
        .id();
    // Delete bucket
    client
        .delete(format!("{}/{}", URL_BUCKET, bucket_id))
        .dispatch();
    // Try reading
    let response = client
        .get(format!("{}/{}", URL_BUCKET, bucket_id))
        .dispatch();
    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn test_bucket_update() {
    // Setup test
    let client = &Setup::new().client;
    // Create an bucket and get back id
    let bucket_form = BucketForm::new(String::from("bucket_name"));
    let bucket_id = client
        .post(URL_BUCKET)
        .json(&bucket_form)
        .dispatch()
        .into_json::<Bucket>()
        .unwrap()
        .id();
    // Update bucket
    let new_bucket = BucketForm::new(String::from("new_name"));
    let response_update = client
        .put(format!("{}/{}", URL_BUCKET, bucket_id))
        .json(&new_bucket)
        .dispatch();
    assert_eq!(response_update.status(), Status::Ok);
    let returned_bucket = response_update.into_json::<Bucket>().unwrap();
    assert_eq!(returned_bucket.as_form(), new_bucket);
    assert_eq!(returned_bucket.id(), bucket_id);
    // Read
    let response_read = client
        .get(format!("{}/{}", URL_BUCKET, bucket_id))
        .dispatch();
    assert_eq!(response_read.status(), Status::Ok);
    assert_eq!(response_read.into_json::<BucketForm>(), Some(new_bucket));
}

#[test]
fn test_bucket_destroy() {
    let client = &Setup::new().client;
    assert_eq!(client.delete(URL_BUCKET).dispatch().status(), Status::Ok);
    assert_eq!(
        client.get(URL_BUCKET).dispatch().into_json::<Vec<Bucket>>(),
        Some(vec![])
    );
}
