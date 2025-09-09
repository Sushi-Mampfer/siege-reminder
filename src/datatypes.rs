use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Data {
    pub username: String,
    pub projects: Vec<Project>,
    pub primary: String,
    pub settings: Settings,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub time: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Settings {
    pub monday: (i64, i32),
    pub tuesday: (i64, i32),
    pub wednesday: (i64, i32),
    pub thursday: (i64, i32),
    pub friday: (i64, i32),
    pub saturday: (i64, i32),
    pub sunday: (i64, i32),
}
