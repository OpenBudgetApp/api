use crate::models;
use crate::schema;

use chrono::NaiveDate;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::fairing::AdHoc;
use rocket::response::status::{Conflict, Created, NotFound};
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, routes};

use crate::DbConnection;
use models::{Transaction, TransactionForm};

#[get("/")]
async fn list(db: DbConnection) -> Json<Vec<Transaction>> {
    db.run(|conn| schema::transactions::table.load::<Transaction>(conn))
        .await
        .map(Json)
        .unwrap()
}

#[get("/<id>")]
async fn read(db: DbConnection, id: i32) -> Result<Json<Transaction>, NotFound<&'static str>> {
    db.run(move |conn| {
        schema::transactions::table
            .filter(schema::transactions::id.eq(id))
            .first::<Transaction>(conn)
    })
    .await
    .map_err(|_| NotFound("Transaction not found."))
    .map(Json)
}

#[post("/", data = "<form>")]
async fn create(
    db: DbConnection,
    form: Json<TransactionForm>,
) -> Result<Created<Json<Transaction>>, Conflict<String>> {
    db.run(move |conn| {
        diesel::insert_into(schema::transactions::table)
            .values(&*form)
            .execute(conn)
    })
    .await
    .map_err(|e| Conflict(Some(e.to_string())))?;
    Ok(Created::new("/").body(get_last_transaction(&db).await.map(Json).unwrap()))
}

#[delete("/<id>")]
async fn delete(db: DbConnection, id: i32) {
    db.run(move |conn| {
        diesel::delete(schema::transactions::table)
            .filter(schema::transactions::id.eq(id))
            .execute(conn)
    })
    .await
    .unwrap();
}

#[put("/<id>", data = "<form>")]
async fn update(db: DbConnection, form: Json<TransactionForm>, id: i32) -> Json<Transaction> {
    db.run(move |conn| {
        diesel::update(schema::transactions::table)
            .filter(schema::transactions::id.eq(id))
            .set(&*form)
            .execute(conn)
            .unwrap();
        schema::transactions::table
            .filter(schema::transactions::id.eq(id))
            .first::<Transaction>(conn)
    })
    .await
    .map(Json)
    .unwrap()
}

#[delete("/")]
async fn destroy(db: DbConnection) {
    db.run(|conn| diesel::delete(schema::transactions::table).execute(conn))
        .await
        .unwrap();
}

#[get("/account/<account_id>/transactions")]
async fn read_transactions_for_account(
    db: DbConnection,
    account_id: i32,
) -> Json<Vec<Transaction>> {
    db.run(move |conn| {
        schema::transactions::table
            .filter(schema::transactions::account_id.eq(account_id))
            .load::<Transaction>(conn)
    })
    .await
    .map(Json)
    .unwrap()
}

#[get("/account/<account_id>/transactions/<year>/<month>")]
async fn read_transactions_for_account_for_period(
    db: DbConnection,
    account_id: i32,
    year: i32,
    month: u8,
) -> Json<Vec<Transaction>> {
    let from_date = NaiveDate::from_ymd(year, month.into(), 1).and_hms(0, 0, 0);
    let to_date = NaiveDate::from_ymd(year, month as u32 + 1, 1).and_hms(0, 0, 0);
    db.run(move |conn| {
        schema::transactions::table
            .filter(schema::transactions::account_id.eq(account_id))
            .filter(schema::transactions::date.ge(from_date))
            .filter(schema::transactions::date.lt(to_date))
            .load::<Transaction>(conn)
    })
    .await
    .map(Json)
    .unwrap()
}

#[get("/bucket/<id>/transactions")]
async fn read_transactions_for_bucket(db: DbConnection, id: i32) -> Json<Vec<Transaction>> {
    db.run(move |conn| {
        schema::transactions::table
            .filter(schema::transactions::bucket_id.eq(id))
            .load::<Transaction>(conn)
    })
    .await
    .map(Json)
    .unwrap()
}

#[get("/bucket/<id>/transactions/<year>/<month>")]
async fn read_transactions_for_bucket_for_period(
    db: DbConnection,
    id: i32,
    year: i32,
    month: u8,
) -> Json<Vec<Transaction>> {
    let from_date = NaiveDate::from_ymd(year, month.into(), 1).and_hms(0, 0, 0);
    let to_date = NaiveDate::from_ymd(year, month as u32 + 1, 1).and_hms(0, 0, 0);
    db.run(move |conn| {
        schema::transactions::table
            .filter(schema::transactions::bucket_id.eq(id))
            .filter(schema::transactions::date.ge(from_date))
            .filter(schema::transactions::date.lt(to_date))
            .load::<Transaction>(conn)
    })
    .await
    .map(Json)
    .unwrap()
}

// While diesel 2.0.0 isn't compatible with Rocket, we can't use `get_result`
// Currently replacing this function manually
async fn get_last_transaction(db: &DbConnection) -> Result<Transaction, diesel::result::Error> {
    db.run(|conn| {
        schema::transactions::table
            .order(schema::transactions::id.desc())
            .first::<Transaction>(conn)
    })
    .await
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Transaction CRUD", |rocket| async {
        rocket
            .mount(
                "/transaction",
                routes![read, create, list, delete, update, destroy],
            )
            .mount(
                "/",
                routes![
                    read_transactions_for_account,
                    read_transactions_for_account_for_period
                ],
            )
            .mount(
                "/",
                routes![
                    read_transactions_for_bucket,
                    read_transactions_for_bucket_for_period
                ],
            )
    })
}
