use crate::{models::take_model::Take, repository::mongodb_repos::MongoRepo};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};
use std::str::FromStr;

#[post("/create", data = "<new_take>")]
pub fn create_take(
    db: &State<MongoRepo>,
    new_take: Json<Take>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = Take {
        id: None,
        token: new_take.token.to_owned(),
        take: new_take.take.to_owned(),
        owner: new_take.owner.to_owned(),
        likes: new_take.likes.to_owned(),
    };
    let take_detail = db.create_take(data);
    match take_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/take/<path>")]
pub fn get_take(db: &State<MongoRepo>, path: String) -> Result<Json<Take>, Status> {
    let id = match i32::from_str(&path) {
        Ok(id) => id,
        Err(_) => return Err(Status::BadRequest),
    };
    let take_detail = db.get_take(&id);

    match take_detail {
        Ok(take) => Ok(Json(take)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/like_take/<path>")]
pub fn like_take(db: &State<MongoRepo>, path: String) -> Result<Json<Take>, Status> {
    let id = match i32::from_str(&path) {
        Ok(id) => id,
        Err(_) => return Err(Status::BadRequest),
    };

    let update_result = db.like_take(&id);
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_take_info = db.get_take(&id);
                return match updated_take_info {
                    Ok(take) => Ok(Json(take)),
                    Err(_) => Err(Status::InternalServerError),
                };
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/unlike_take/<path>")]
pub fn unlike_take(db: &State<MongoRepo>, path: String) -> Result<Json<Take>, Status> {
    let id = match i32::from_str(&path) {
        Ok(id) => id,
        Err(_) => return Err(Status::BadRequest),
    };
    let update_result = db.unlike_take(&id);
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_take_info = db.get_take(&id);
                return match updated_take_info {
                    Ok(take) => Ok(Json(take)),
                    Err(_) => Err(Status::InternalServerError),
                };
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
