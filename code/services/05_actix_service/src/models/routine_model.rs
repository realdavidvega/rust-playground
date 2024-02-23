use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Lane {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    fromHour: String,
    toHour: String,
    color: String,
    locked: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Routine {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub lanes: Vec<Lane>
}