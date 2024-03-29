use crate::models;
use crate::schema;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::fairing::AdHoc;
use rocket::response::status::{Conflict, Created, NotFound};
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, routes};

use crate::DbConnection;
use models::{Account, AccountForm};

#[get("/")]
async fn list(db: DbConnection) -> Json<Vec<Account>> {
    db.run(|conn| schema::accounts::table.load::<Account>(conn))
        .await
        .map(Json)
        .unwrap()
}

#[get("/<account_id>")]
async fn read(db: DbConnection, account_id: i32) -> Result<Json<Account>, NotFound<&'static str>> {
    db.run(move |conn| {
        schema::accounts::table
            .filter(schema::accounts::id.eq(account_id))
            .first::<Account>(conn)
    })
    .await
    .map_err(|_| NotFound("Account not found."))
    .map(Json)
}

#[post("/", data = "<account_form>")]
async fn create(
    db: DbConnection,
    account_form: Json<AccountForm>,
) -> Result<Created<Json<Account>>, Conflict<&'static str>> {
    db.run(move |conn| {
        diesel::insert_into(schema::accounts::table)
            .values(&*account_form)
            .execute(conn)
    })
    .await
    .map_err(|_| Conflict(Some("Account already exists.")))?;
    Ok(Created::new("/").body(get_last_account(&db).await.map(Json).unwrap()))
}

#[delete("/<account_id>")]
async fn delete(db: DbConnection, account_id: i32) -> Result<(), Conflict<String>> {
    db.run(move |conn| {
        diesel::delete(schema::accounts::table)
            .filter(schema::accounts::id.eq(account_id))
            .execute(conn)
    })
    .await
    .map_err(|e| Conflict(Some(e.to_string())))?;
    Ok(())
}

#[put("/<account_id>", data = "<account_form>")]
async fn update(
    db: DbConnection,
    account_form: Json<AccountForm>,
    account_id: i32,
) -> Json<Account> {
    db.run(move |conn| {
        diesel::update(schema::accounts::table)
            .filter(schema::accounts::id.eq(account_id))
            .set(&*account_form)
            .execute(conn)
            .unwrap();
        schema::accounts::table
            .filter(schema::accounts::id.eq(account_id))
            .first::<Account>(conn)
    })
    .await
    .map(Json)
    .unwrap()
}

#[delete("/")]
async fn destroy(db: DbConnection) {
    db.run(|conn| diesel::delete(schema::accounts::table).execute(conn))
        .await
        .unwrap();
}

// While diesel 2.0.0 isn't compatible with Rocket, we can't use `get_result`
// Currently replacing this function manually
async fn get_last_account(db: &DbConnection) -> Result<Account, diesel::result::Error> {
    db.run(|conn| {
        schema::accounts::table
            .order(schema::accounts::id.desc())
            .first::<Account>(conn)
    })
    .await
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Account CRUD", |rocket| async {
        rocket.mount(
            "/account",
            routes![read, create, list, delete, update, destroy],
        )
    })
}
