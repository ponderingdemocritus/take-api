use std::env;
extern crate dotenv;
use dotenv::dotenv;

use crate::models::take_model::Take;
use mongodb::{
    bson::{doc, extjson::de::Error},
    results::{InsertOneResult, UpdateResult}, //modify here
    sync::{Client, Collection},
};

pub struct MongoRepo {
    col: Collection<Take>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustDB");
        let col: Collection<Take> = db.collection("User");
        MongoRepo { col }
    }

    pub fn create_take(&self, new_take: Take) -> Result<InsertOneResult, Error> {
        let new_doc = Take {
            id: None,
            token: new_take.token,
            take: new_take.take,
            owner: new_take.owner,
            likes: new_take.likes,
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating user");
        Ok(user)
    }

    pub fn get_take(&self, token: &i32) -> Result<Take, Error> {
        let filter = doc! {"token": token};
        let user_detail = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }

    pub fn like_take(&self, token: &i32) -> Result<UpdateResult, Error> {
        let filter = doc! {"token": token};
        let new_doc = doc! {
             "$inc": { "likes": 1 }
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }
    pub fn unlike_take(&self, token: &i32) -> Result<UpdateResult, Error> {
        let filter = doc! {"token": token};
        let new_doc = doc! {
             "$inc": { "likes": -1 }
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }
}
