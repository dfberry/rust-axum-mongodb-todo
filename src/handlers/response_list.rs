use chrono::{DateTime, Utc};
use serde::Serialize;

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct ListResponse {
    pub id: String,
    pub name: String,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
pub struct ListData {
    pub list: ListResponse,
}

#[derive(Serialize, Debug)]
pub struct SingleListResponse {
    pub status: &'static str,
    pub data: ListData,
}

#[derive(Serialize, Debug)]
pub struct ListListResponse {
    pub status: &'static str,
    pub results: usize,
    pub lists: Vec<ListResponse>,
}
