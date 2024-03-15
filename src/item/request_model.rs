use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewItemRequestModel {
    pub listId: String,
    pub name: String,
    pub state: Option<String>,
    pub description: Option<String>,
    pub dueDate: Option<String>,

}