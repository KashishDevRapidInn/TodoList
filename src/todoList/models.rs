use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateEntryData{
    pub title:String,
    pub date: i64
}

#[derive(Deserialize, Clone)]
pub struct UpdateEntryData{
    pub title:String
}