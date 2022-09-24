use crate::models;
use crate::schema;

use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::fairing::AdHoc;
use rocket::response::status::{Conflict, Created, NotFound};
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, routes};

use crate::DbConnection;
use models::{Bucket, BucketForm};

#[get("/")]
async fn list(db: DbConnection) -> Json<Vec<Bucket>> {
    db.run(|conn| schema::buckets::table.load::<Bucket>(conn))
        .await
        .map(Json)
        .unwrap()
}

#[get("/<id>")]
async fn read(db: DbConnection, id: i32) -> Result<Json<Bucket>, NotFound<String>> {
    db.run(move |conn| {
        schema::buckets::table
            .filter(schema::buckets::id.eq(id))
            .first::<Bucket>(conn)
    })
    .await
    .map_err(|e| NotFound(e.to_string()))
    .map(Json)
}

#[post("/", data = "<form>")]
async fn create(
    db: DbConnection,
    form: Json<BucketForm>,
) -> Result<Created<Json<Bucket>>, Conflict<String>> {
    db.run(move |conn| {
        diesel::insert_into(schema::buckets::table)
            .values(&*form)
            .execute(conn)
    })
    .await
    .map_err(|e| Conflict(Some(e.to_string())))?;
    Ok(Created::new("/").body(get_last_bucket(&db).await.map(Json).unwrap()))
}

#[delete("/<id>")]
async fn delete(db: DbConnection, id: i32) -> Result<(), Conflict<String>> {
    db.run(move |conn| {
        diesel::delete(schema::buckets::table)
            .filter(schema::buckets::id.eq(id))
            .execute(conn)
    })
    .await
    .map_err(|e| Conflict(Some(e.to_string())))?;
    Ok(())
}

#[put("/<id>", data = "<form>")]
async fn update(db: DbConnection, form: Json<BucketForm>, id: i32) -> Json<Bucket> {
    db.run(move |conn| {
        diesel::update(schema::buckets::table)
            .filter(schema::buckets::id.eq(id))
            .set(&*form)
            .execute(conn)
            .unwrap();
        schema::buckets::table
            .filter(schema::buckets::id.eq(id))
            .first::<Bucket>(conn)
    })
    .await
    .map(Json)
    .unwrap()
}

#[delete("/")]
async fn destroy(db: DbConnection) {
    db.run(|conn| diesel::delete(schema::buckets::table).execute(conn))
        .await
        .unwrap();
}

// While diesel 2.0.0 isn't compatible with Rocket, we can't use `get_result`
// Currently replacing this function manually
async fn get_last_bucket(db: &DbConnection) -> Result<Bucket, diesel::result::Error> {
    db.run(|conn| {
        schema::buckets::table
            .order(schema::buckets::id.desc())
            .first::<Bucket>(conn)
    })
    .await
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Bucket CRUD", |rocket| async {
        rocket.mount(
            "/bucket",
            routes![read, create, list, delete, update, destroy],
        )
    })
}
