use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateItemSchema {
    pub listId: String,
    pub name: String,
    pub state: String,
    pub description: Option<String>,
    pub dueDate: Option<String>,
    pub completedDate: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateItemSchema {
    pub listId: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dueDate: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completedDate: Option<DateTime<Utc>>,
}