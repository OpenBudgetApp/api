use crate::models;
use crate::schema;

use chrono::NaiveDate;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::fairing::AdHoc;
use rocket::response::status::{Conflict, Created, NotFound};
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, routes};

use crate::DbConnection;
use models::{Fill, FillForm};

#[get("/")]
async fn list(db: DbConnection) -> Json<Vec<Fill>> {
    db.run(|conn| schema::fills::table.load::<Fill>(conn))
        .await
        .map(Json)
        .unwrap()
}

#[get("/<id>")]
async fn read(db: DbConnection, id: i32) -> Result<Json<Fill>, NotFound<&'static str>> {
    db.run(move |conn| {
        schema::fills::table
            .filter(schema::fills::id.eq(id))
            .first::<Fill>(conn)
    })
    .await
    .map_err(|_| NotFound("Fill not found."))
    .map(Json)
}

#[post("/", data = "<form>")]
async fn create(
    db: DbConnection,
    form: Json<FillForm>,
) -> Result<Created<Json<Fill>>, Conflict<String>> {
    db.run(move |conn| {
        diesel::insert_into(schema::fills::table)
            .values(&*form)
            .execute(conn)
    })
    .await
    .map_err(|e| Conflict(Some(e.to_string())))?;
    Ok(Created::new("/").body(get_last_fill(&db).await.map(Json).unwrap()))
}

#[delete("/<id>")]
async fn delete(db: DbConnection, id: i32) {
    db.run(move |conn| {
        diesel::delete(schema::fills::table)
            .filter(schema::fills::id.eq(id))
            .execute(conn)
    })
    .await
    .unwrap();
}

#[put("/<id>", data = "<form>")]
async fn update(db: DbConnection, form: Json<FillForm>, id: i32) -> Json<Fill> {
    db.run(move |conn| {
        diesel::update(schema::fills::table)
            .filter(schema::fills::id.eq(id))
            .set(&*form)
            .execute(conn)
            .unwrap();
        schema::fills::table
            .filter(schema::fills::id.eq(id))
            .first::<Fill>(conn)
    })
    .await
    .map(Json)
    .unwrap()
}

#[delete("/")]
async fn destroy(db: DbConnection) {
    db.run(|conn| diesel::delete(schema::fills::table).execute(conn))
        .await
        .unwrap();
}

#[get("/bucket/<id>/fills")]
async fn read_fills_for_bucket(db: DbConnection, id: i32) -> Json<Vec<Fill>> {
    db.run(move |conn| {
        schema::fills::table
            .filter(schema::fills::bucket_id.eq(id))
            .load::<Fill>(conn)
    })
    .await
    .map(Json)
    .unwrap()
}

#[get("/bucket/<id>/fills/<year>/<month>")]
async fn read_fills_for_bucket_for_period(
    db: DbConnection,
    id: i32,
    year: i32,
    month: u8,
) -> Json<Vec<Fill>> {
    let from_date = NaiveDate::from_ymd(year, month.into(), 1).and_hms(0, 0, 0);
    let to_date = NaiveDate::from_ymd(year, month as u32 + 1, 1).and_hms(0, 0, 0);
    db.run(move |conn| {
        schema::fills::table
            .filter(schema::fills::bucket_id.eq(id))
            .filter(schema::fills::date.ge(from_date))
            .filter(schema::fills::date.lt(to_date))
            .load::<Fill>(conn)
    })
    .await
    .map(Json)
    .unwrap()
}

// While diesel 2.0.0 isn't compatible with Rocket, we can't use `get_result`
// Currently replacing this function manually
async fn get_last_fill(db: &DbConnection) -> Result<Fill, diesel::result::Error> {
    db.run(|conn| {
        schema::fills::table
            .order(schema::fills::id.desc())
            .first::<Fill>(conn)
    })
    .await
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Fill CRUD", |rocket| async {
        rocket
            .mount(
                "/fill",
                routes![read, create, list, delete, update, destroy],
            )
            .mount(
                "/",
                routes![read_fills_for_bucket, read_fills_for_bucket_for_period],
            )
    })
}
