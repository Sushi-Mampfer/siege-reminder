use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Data {
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
    pub monday: String,
    pub tuesday: String,
    pub wednesday: String,
    pub thursday: String,
    pub friday: String,
    pub saturday: String,
    pub sunday: String,
}
