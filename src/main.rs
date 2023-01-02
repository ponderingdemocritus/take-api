mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use api::take_api::{create_take, get_take, like_take, unlike_take};
use repository::mongodb_repos::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![create_take])
        .mount("/", routes![get_take])
        .mount("/", routes![like_take])
        .mount("/", routes![unlike_take])
}
