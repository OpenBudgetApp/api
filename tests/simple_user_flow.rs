mod common;

use chrono::NaiveDateTime;
use rocket::local::blocking::Client;

use common::Setup;
use common::{Account, Bucket, Fill, Transaction};
use common::{URL_ACCOUNT, URL_BUCKET, URL_FILL, URL_TRANSACTION};

fn create_bucket_from_name(client: &Client, name: String) -> i32 {
    client
        .post(URL_BUCKET)
        .json(&Bucket::new(name))
        .dispatch()
        .into_json::<Bucket>()
        .unwrap()
        .id
        .unwrap()
}

#[test]
fn test_simple_user_flow() {
    // Setup test
    let setup = Setup::new();
    let client = &setup.client;
    // Create an account
    let account_id = client
        .post(URL_ACCOUNT)
        .json(&Account {
            id: None,
            name: String::from("banking"),
        })
        .dispatch()
        .into_json::<Account>()
        .unwrap()
        .id
        .unwrap();
    // Create a positive transaction (income)
    let date = NaiveDateTime::parse_from_str("2022-07-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    client
        .post(URL_TRANSACTION)
        .json(&Transaction::new(
            String::from("Income"),
            1300.0,
            date,
            account_id,
            None,
        ))
        .dispatch();
    // Create three buckets and fill them
    let bucket_bills_id = create_bucket_from_name(client, String::from("Bills"));
    client
        .post(URL_FILL)
        .json(&Fill::new(950.0, date, bucket_bills_id))
        .dispatch();
    let bucket_food_id = create_bucket_from_name(client, String::from("Food"));
    client
        .post(URL_FILL)
        .json(&Fill::new(150.0, date, bucket_food_id))
        .dispatch();
    let bucket_holidays_id = create_bucket_from_name(client, String::from("Holidays"));
    client
        .post(URL_FILL)
        .json(&Fill::new(200.0, date, bucket_holidays_id))
        .dispatch();
    // Here we check the sum of positive transactions is equal to the sum of fills
    check_bucket_sum_less_income(client, account_id);
    // Start making payments
    let transactions = vec![
        Transaction::new(
            String::from("Rent"),
            -800.0,
            date,
            account_id,
            Some(bucket_bills_id),
        ),
        Transaction::new(
            String::from("Groceries"),
            -30.0,
            date,
            account_id,
            Some(bucket_food_id),
        ),
        Transaction::new(
            String::from("Electricity"),
            -100.0,
            date,
            account_id,
            Some(bucket_bills_id),
        ),
    ];
    for transaction in &transactions {
        client.post(URL_TRANSACTION).json(transaction).dispatch();
    }
    check_bucket_sum_less_income(client, account_id);
    // Check that 50 is left on the bills bucket
    assert_eq!(
        get_bucket_fill(client, bucket_bills_id) + get_bucket_consumption(client, bucket_bills_id),
        50.0
    );
}

fn get_bucket_fill(client: &Client, bucket_id: i32) -> f32 {
    client
        .get(format!("{}/{}/fills/2022/07", URL_BUCKET, bucket_id,))
        .dispatch()
        .into_json::<Vec<Fill>>()
        .unwrap()
        .iter()
        .map(|fill| fill.amount)
        .sum::<f32>()
}

fn get_bucket_consumption(client: &Client, bucket_id: i32) -> f32 {
    client
        .get(format!("{}/{}/transactions/2022/07", URL_BUCKET, bucket_id,))
        .dispatch()
        .into_json::<Vec<Transaction>>()
        .unwrap()
        .iter()
        .map(|fill| fill.amount)
        .sum::<f32>()
}

fn check_bucket_sum_less_income(client: &Client, account_id: i32) {
    let buckets = client
        .get(URL_BUCKET)
        .dispatch()
        .into_json::<Vec<Bucket>>()
        .unwrap();
    let mut sum_fills = 0.0;
    for bucket in &buckets {
        sum_fills += client
            .get(format!(
                "{}/{}/fills/2022/07",
                URL_BUCKET,
                bucket.id.unwrap(),
            ))
            .dispatch()
            .into_json::<Vec<Fill>>()
            .unwrap()
            .iter()
            .map(|fill| fill.amount)
            .sum::<f32>();
    }
    let sum_transactions = client
        .get(format!(
            "{}/{}/transactions/2022/07",
            URL_ACCOUNT, account_id,
        ))
        .dispatch()
        .into_json::<Vec<Transaction>>()
        .unwrap()
        .iter()
        .map(|fill| fill.amount)
        .filter(|x| x.is_sign_positive())
        .sum::<f32>();
    assert!(sum_fills <= sum_transactions);
}
