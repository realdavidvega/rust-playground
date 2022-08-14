extern crate dotenv;

use std::env;
use dotenv::dotenv;
use mongodb::{
    bson::extjson::de::Error,
    results::InsertOneResult,
    Client, Collection,
};

use crate::models::routine_model::Routine;

pub struct MongoRepo {
    col: Collection<Routine>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("my-db");
        let col: Collection<Routine> = db.collection("Routines");
        MongoRepo { col }
    }

    pub async fn create_routine(&self, new_routine: Routine) -> Result<InsertOneResult, Error> {
        let item = Routine {
            id: None,
            lanes: new_routine.lanes
        };
        let routine = self
            .col
            .insert_one(item, None)
            .await
            .ok()
            .expect("Error creating routine");
        Ok(routine)
    }
}