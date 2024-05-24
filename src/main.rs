#[macro_use] extern crate rocket;

mod models;
mod routes;
mod errors;

use rocket::fs::{FileServer, relative};
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    rocket::build()
        .manage(pool)
        .mount("/api", routes![routes::get_notes, routes::create_note])
        .mount("/", FileServer::from(relative!("static")))
}
