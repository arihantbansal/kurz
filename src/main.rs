#[macro_use]
extern crate rocket;

use rocket::{http::Status, response::Redirect, State};
use sqlx::postgres::PgPool;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<id>")]
async fn redirect(id: String, pool: &State<PgPool>) -> Result<Redirect, Status> {
    let url: (String,) = sqlx::query_as("SELECT url FROM urls WHERE id = $1")
        .bind(id)
        .fetch_one(&**pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => Status::NotFound,
            _ => Status::InternalServerError,
        })?;

    Ok(Redirect::to(url.0))
}

#[post("/", data = "<url>")]
async fn shorten(url: String, pool: &State<PgPool>) -> Result<String, Status> {
    let id = &nanoid::nanoid!(6);
    let p_url = Url::parse(&url).map_err(|_| Status::UnprocessableEntity);
    sqlx::query("INSERT INTO urls(id, url) VALUES ($1, $2)")
        .bind(id)
        .bind(p_url.as_str())
        .execute(&**pool)
        .await
        .map_err(|_| Status::InternalServerError)?;
    Ok(format!("https://kurz.shuttleapp.rs/{id}"))
}

#[shuttle_runtime::main]
async fn rocket(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![index]);

    Ok(rocket.into())
}
