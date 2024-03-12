use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateListSchema {
    pub name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateListSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>
}