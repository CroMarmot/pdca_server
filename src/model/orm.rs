use serde::{self, Deserialize, Serialize};

use mongodb::bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug)]
pub struct DailyPDCA {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    pub date: String,
    pub plan_and_do: Vec<PD>,
    pub check: String,
    pub action: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PD {
    pub start_time: String,
    pub end_time: String,
    pub plan: String,
    pub finished: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
