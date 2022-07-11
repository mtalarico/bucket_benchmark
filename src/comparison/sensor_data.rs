use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SensorData {
    pub _id: ObjectId,
    pub ts: DateTime,
    pub data: i32,
    pub thread: i32,
}

impl SensorData {
    pub fn new(data: i32, thread: i32) -> SensorData {
        return SensorData {
            _id: ObjectId::new(),
            ts: DateTime::now(),
            data,
            thread,
        };
    }
}
