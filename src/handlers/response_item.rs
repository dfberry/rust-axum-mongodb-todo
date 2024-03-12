use chrono::{DateTime, Utc};
use serde::Serialize;

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct ItemResponse {
    pub id: String,
    pub listId: String,
    pub name: String,
    pub state: String,
    pub description: Option<String>,
    pub dueDate: Option<DateTime<Utc>>,
    pub completedDate: Option<DateTime<Utc>>
}

#[derive(Serialize, Debug)]
pub struct ItemData {
    pub item: ItemResponse,
}

#[derive(Serialize, Debug)]
pub struct SingleItemResponse {
    pub status: &'static str,
    pub data: ItemData,
}

#[derive(Serialize, Debug)]
pub struct ItemListResponse {
    pub status: &'static str,
    pub results: usize,
    pub items: Vec<ItemResponse>,
}
