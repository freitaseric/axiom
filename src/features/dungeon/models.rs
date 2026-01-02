use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Boss {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub guild_id: String,
    pub name: String,
    pub hp: i32,
    pub max_hp: i32,
    pub active: bool,
    pub thread_id: Option<String>,
}
