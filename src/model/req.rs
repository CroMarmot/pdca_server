use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DailyKey {
    pub date: Option<String>,
    pub _id: Option<String>,
}
